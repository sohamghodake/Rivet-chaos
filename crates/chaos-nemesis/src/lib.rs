//! Fault injection through the Docker API. Each fault is written to the history
//! as a `fault-start` just before it's applied and a `fault-heal` once it's
//! undone, so the checker knows every fault's window.

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, Instant as StdInstant},
};

use anyhow::{Context, Result, bail};
use bollard::{
    Docker,
    models::{EndpointSettings, NetworkConnectRequest, NetworkDisconnectRequest},
    query_parameters::{
        KillContainerOptionsBuilder, ListContainersOptionsBuilder, RemoveContainerOptionsBuilder,
        RemoveVolumeOptionsBuilder,
    },
};
use chaos_core::{Body, Fault, FaultSpec, Network, Proc};
use chaos_history::Recorder;
use rand::{Rng, SeedableRng, rngs::StdRng};
use serde::Deserialize;
use tokio::{task::JoinSet, time::Instant};
use tokio_util::sync::CancellationToken;

/// How long `kill-in-step` waits for a workflow step on one of its nodes before
/// giving up on that occurrence.
const STEP_WAIT: Duration = Duration::from_secs(120);
/// How long a kill or pause waits for its target to be running, as when an
/// engine that fenced itself is being restarted, before skipping that occurrence.
const TARGET_WAIT: Duration = Duration::from_secs(30);

/// One fault, at a fixed offset from the start of the run.
#[derive(Debug, Clone, PartialEq)]
pub struct Planned {
    pub at: Duration,
    pub heal_after: Duration,
    pub fault: Fault,
    /// The service to fault. Empty for `kill-in-step`, whose step picks the node.
    pub target: String,
    pub network: Option<Network>,
    /// For `kill-in-step`, the nodes it may kill.
    pub candidates: Vec<String>,
}

/// Expands every spec's repeats and picks each occurrence's target with the
/// run's seed, so the same seed always faults the same targets in the same order.
pub fn plan(specs: &[FaultSpec], duration_s: u64, seed: u64) -> Vec<Planned> {
    let mut rng = StdRng::seed_from_u64(seed ^ 0x006e_656d_6573_6973);
    let mut planned: Vec<Planned> = specs
        .iter()
        .flat_map(|spec| {
            spec.start_times(duration_s)
                .into_iter()
                .map(move |at| (spec, at))
        })
        .map(|(spec, at)| {
            let choices = spec.target.choices();
            let (target, candidates) = match spec.fault {
                Fault::KillInStep => (String::new(), choices.to_vec()),
                _ => (
                    choices[rng.random_range(0..choices.len())].clone(),
                    Vec::new(),
                ),
            };
            Planned {
                at: Duration::from_secs(at),
                heal_after: Duration::from_secs(spec.heal_after_s),
                fault: spec.fault,
                target,
                network: spec.network,
                candidates,
            }
        })
        .collect();
    planned.sort_by_key(|fault| fault.at);
    planned
}

#[derive(Debug, Deserialize)]
struct StepCall {
    key: String,
    node: String,
}

#[derive(Clone)]
pub struct Nemesis {
    docker: Docker,
    project: String,
    compose_file: PathBuf,
    sink: String,
    http: reqwest::Client,
    /// A disconnected container's settings on its network, to reconnect it with
    /// the same address and DNS names.
    detached: Arc<Mutex<HashMap<(String, Network), EndpointSettings>>>,
}

impl Nemesis {
    /// `compose_file` recreates a replaced service; `sink` arms `kill-in-step`.
    pub fn connect(project: &str, compose_file: impl Into<PathBuf>, sink: &str) -> Result<Self> {
        Ok(Self {
            docker: Docker::connect_with_local_defaults().context("connect to Docker")?,
            project: project.into(),
            compose_file: compose_file.into(),
            sink: sink.trim_end_matches('/').into(),
            http: reqwest::Client::new(),
            detached: Arc::default(),
        })
    }

    pub fn docker(&self) -> &Docker {
        &self.docker
    }

    /// The container behind a Compose service, found by its Compose labels.
    pub async fn container(&self, service: &str) -> Result<String> {
        let labels = vec![
            format!("com.docker.compose.project={}", self.project),
            format!("com.docker.compose.service={service}"),
        ];
        let filters = HashMap::from([("label", labels)]);
        let options = ListContainersOptionsBuilder::new()
            .all(true)
            .filters(&filters)
            .build();
        let containers = self.docker.list_containers(Some(options)).await?;
        match containers.as_slice() {
            [container] => container.id.clone().context("container has no ID"),
            [] => bail!(
                "no container for service {service} in project {}",
                self.project
            ),
            many => bail!("{} containers for service {service}", many.len()),
        }
    }

    fn network_name(&self, network: Network) -> String {
        format!("{}_{network}", self.project)
    }

    /// Whether the service's container is running and not paused, waiting up to
    /// `wait` for it to become so.
    async fn running(&self, service: &str, wait: Duration) -> Result<bool> {
        let started = StdInstant::now();
        loop {
            let id = self.container(service).await?;
            let state = self
                .docker
                .inspect_container(&id, None)
                .await?
                .state
                .unwrap_or_default();
            if state.running == Some(true) && state.paused != Some(true) {
                return Ok(true);
            }
            if started.elapsed() >= wait {
                return Ok(false);
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    async fn sigkill(&self, id: &str) -> Result<()> {
        let options = KillContainerOptionsBuilder::new().signal("SIGKILL").build();
        Ok(self.docker.kill_container(id, Some(options)).await?)
    }

    async fn apply(&self, fault: Fault, target: &str, network: Option<Network>) -> Result<()> {
        let id = self.container(target).await?;
        match fault {
            Fault::Kill | Fault::KillInStep => self.sigkill(&id).await?,
            Fault::Pause => self.docker.pause_container(&id).await?,
            Fault::Replace => {
                let volumes: Vec<String> = self
                    .docker
                    .inspect_container(&id, None)
                    .await?
                    .mounts
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|mount| mount.typ.as_deref() == Some("volume"))
                    .filter_map(|mount| mount.name)
                    .collect();
                self.sigkill(&id).await?;
                let options = RemoveContainerOptionsBuilder::new()
                    .force(true)
                    .v(true)
                    .build();
                self.docker.remove_container(&id, Some(options)).await?;
                for volume in volumes {
                    let options = RemoveVolumeOptionsBuilder::new().force(true).build();
                    self.docker.remove_volume(&volume, Some(options)).await?;
                }
            }
            Fault::Disconnect => {
                let network = network.context("disconnect needs a network")?;
                let name = self.network_name(network);
                let settings = self
                    .docker
                    .inspect_container(&id, None)
                    .await?
                    .network_settings
                    .and_then(|settings| settings.networks)
                    .and_then(|mut networks| networks.remove(&name))
                    .with_context(|| format!("{target} is not on {name}"))?;
                self.detached
                    .lock()
                    .unwrap()
                    .insert((target.into(), network), settings);
                let request = NetworkDisconnectRequest {
                    container: id,
                    force: Some(true),
                };
                self.docker.disconnect_network(&name, request).await?;
            }
        }
        Ok(())
    }

    async fn heal(&self, fault: Fault, target: &str, network: Option<Network>) -> Result<()> {
        match fault {
            Fault::Kill | Fault::KillInStep => {
                let id = self.container(target).await?;
                self.docker.start_container(&id, None).await?;
            }
            Fault::Pause => {
                let id = self.container(target).await?;
                self.docker.unpause_container(&id).await?;
            }
            Fault::Replace => {
                // Compose creates the service again from its definition: same
                // address and config, fresh volumes.
                let status = tokio::process::Command::new("docker")
                    .args(["compose", "-p", &self.project, "-f"])
                    .arg(&self.compose_file)
                    .args(["up", "--detach", "--no-deps", "--quiet-pull", target])
                    .status()
                    .await
                    .context("run docker compose")?;
                if !status.success() {
                    bail!("docker compose up {target} exited with {status}");
                }
            }
            Fault::Disconnect => {
                let network = network.context("disconnect needs a network")?;
                let saved = self
                    .detached
                    .lock()
                    .unwrap()
                    .remove(&(target.into(), network))
                    .with_context(|| format!("no saved settings for {target} on {network}"))?;
                let request = NetworkConnectRequest {
                    container: self.container(target).await?,
                    endpoint_config: Some(EndpointSettings {
                        aliases: saved.aliases,
                        ipam_config: saved.ipam_config,
                        ..Default::default()
                    }),
                };
                self.docker
                    .connect_network(&self.network_name(network), request)
                    .await?;
            }
        }
        Ok(())
    }

    /// Arms the sink and waits for a tool call from one of `candidates`, which the
    /// sink holds unanswered so the node can be killed inside the step. `None` if
    /// no such step ran in time.
    async fn wait_for_step(
        &self,
        candidates: &[String],
        stop: &CancellationToken,
    ) -> Result<Option<StepCall>> {
        let started = StdInstant::now();
        while started.elapsed() < STEP_WAIT && !stop.is_cancelled() {
            let wait = self
                .http
                .get(format!("{}/trigger", self.sink))
                .query(&[("timeout_s", "10")])
                .send();
            let response = tokio::select! {
                _ = stop.cancelled() => return Ok(None),
                response = wait => response?.error_for_status()?,
            };
            if response.status() == reqwest::StatusCode::NO_CONTENT {
                continue;
            }
            let call: StepCall = response.json().await?;
            if candidates.contains(&call.node) {
                return Ok(Some(call));
            }
        }
        Ok(None)
    }

    /// Runs the plan against the clock that started at `started`. If `stop` is
    /// cancelled, faults not yet started are skipped and active ones heal at once.
    pub async fn run(
        &self,
        plan: Vec<Planned>,
        recorder: Recorder,
        started: Instant,
        stop: CancellationToken,
    ) -> Result<()> {
        let mut faults = JoinSet::new();
        for planned in plan {
            let (nemesis, recorder, stop) = (self.clone(), recorder.clone(), stop.clone());
            faults.spawn(async move {
                tokio::select! {
                    _ = stop.cancelled() => return anyhow::Ok(()),
                    _ = tokio::time::sleep_until(started + planned.at) => {}
                }
                let (fault, network) = (planned.fault, planned.network);
                let target = match fault {
                    Fault::KillInStep => {
                        match nemesis.wait_for_step(&planned.candidates, &stop).await? {
                            Some(call) => {
                                println!(
                                    "rivet-chaos: killing {} inside the step that called {}",
                                    call.node, call.key
                                );
                                call.node
                            }
                            None => {
                                println!(
                                    "rivet-chaos: no workflow step ran on {:?} in time; skipped",
                                    planned.candidates
                                );
                                return Ok(());
                            }
                        }
                    }
                    _ => planned.target,
                };
                if matches!(fault, Fault::Kill | Fault::Pause)
                    && !nemesis.running(&target, TARGET_WAIT).await?
                {
                    println!(
                        "rivet-chaos: {target} wasn't running, so {fault} {target} was skipped"
                    );
                    return Ok(());
                }
                recorder.record(
                    Proc::Nemesis,
                    Body::FaultStart {
                        fault,
                        target: target.clone(),
                        network,
                    },
                    true,
                )?;
                nemesis
                    .apply(fault, &target, network)
                    .await
                    .with_context(|| format!("{fault} {target}"))?;
                tokio::select! {
                    _ = stop.cancelled() => {}
                    _ = tokio::time::sleep(planned.heal_after) => {}
                }
                nemesis
                    .heal(fault, &target, network)
                    .await
                    .with_context(|| format!("heal {fault} {target}"))?;
                recorder.record(
                    Proc::Nemesis,
                    Body::FaultHeal {
                        fault,
                        target,
                        network,
                    },
                    true,
                )?;
                Ok(())
            });
        }
        while let Some(result) = faults.join_next().await {
            result??;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chaos_core::Targets;

    use super::*;

    fn spec(fault: Fault, targets: &[&str]) -> FaultSpec {
        FaultSpec {
            fault,
            target: Targets::Any(targets.iter().map(|target| target.to_string()).collect()),
            network: None,
            at_s: 60,
            heal_after_s: 30,
            repeat_every_s: Some(120),
        }
    }

    #[test]
    fn plan_repeats_until_the_end_of_the_run() {
        let planned = plan(&[spec(Fault::Kill, &["engine-1"])], 300, 42);
        let starts: Vec<u64> = planned.iter().map(|fault| fault.at.as_secs()).collect();
        assert_eq!(starts, [60, 180]);
        assert!(
            planned
                .iter()
                .all(|fault| fault.heal_after == Duration::from_secs(30))
        );
    }

    #[test]
    fn same_seed_picks_the_same_targets() {
        let specs = [spec(Fault::Kill, &["engine-1", "engine-2", "engine-3"])];
        let targets = |seed| -> Vec<String> {
            plan(&specs, 3_000, seed)
                .into_iter()
                .map(|fault| fault.target)
                .collect()
        };
        assert_eq!(targets(7), targets(7));
        assert_ne!(targets(7), targets(8));
    }

    #[test]
    fn kill_in_step_leaves_the_choice_to_the_step() {
        let planned = plan(&[spec(Fault::KillInStep, &["node-a", "node-b"])], 300, 1);
        assert!(planned.iter().all(|fault| fault.target.is_empty()));
        assert_eq!(planned[0].candidates, ["node-a", "node-b"]);
    }

    #[test]
    fn disconnects_carry_their_network() {
        let mut disconnect = spec(Fault::Disconnect, &["engine-2"]);
        disconnect.network = Some(Network::Peer);
        assert_eq!(plan(&[disconnect], 300, 1)[0].network, Some(Network::Peer));
    }
}
