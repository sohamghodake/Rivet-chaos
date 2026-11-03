//! Fault injection through the Docker API. Each fault is written to the history
//! as a `fault-start` just before it's applied and a `fault-heal` once it's
//! undone, so the checker knows every fault's window.

use std::{collections::HashMap, time::Duration};

use anyhow::{Context, Result, bail};
use bollard::{
    Docker,
    query_parameters::{KillContainerOptionsBuilder, ListContainersOptionsBuilder},
};
use chaos_core::{Body, Fault, FaultSpec, Proc};
use chaos_history::Recorder;
use rand::{Rng, SeedableRng, rngs::StdRng};
use tokio::{task::JoinSet, time::Instant};
use tokio_util::sync::CancellationToken;

/// One fault, at a fixed offset from the start of the run.
#[derive(Debug, Clone, PartialEq)]
pub struct Planned {
    pub at: Duration,
    pub heal_after: Duration,
    pub fault: Fault,
    pub target: String,
}

/// Expands every spec's repeats and picks each occurrence's target with the
/// run's seed, so the same seed always faults the same targets in the same order.
pub fn plan(specs: &[FaultSpec], duration_s: u64, seed: u64) -> Vec<Planned> {
    let mut rng = StdRng::seed_from_u64(seed ^ 0x6e65_6d65_7369_73);
    let mut planned: Vec<Planned> = specs
        .iter()
        .flat_map(|spec| spec.start_times(duration_s).into_iter().map(move |at| (spec, at)))
        .map(|(spec, at)| {
            let choices = spec.target.choices();
            Planned {
                at: Duration::from_secs(at),
                heal_after: Duration::from_secs(spec.heal_after_s),
                fault: spec.fault,
                target: choices[rng.random_range(0..choices.len())].clone(),
            }
        })
        .collect();
    planned.sort_by_key(|fault| fault.at);
    planned
}

#[derive(Clone)]
pub struct Nemesis {
    docker: Docker,
    project: String,
}

impl Nemesis {
    pub fn connect(project: &str) -> Result<Self> {
        Ok(Self {
            docker: Docker::connect_with_local_defaults().context("connect to Docker")?,
            project: project.into(),
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
        let options = ListContainersOptionsBuilder::new().all(true).filters(&filters).build();
        let containers = self.docker.list_containers(Some(options)).await?;
        match containers.as_slice() {
            [container] => container.id.clone().context("container has no ID"),
            [] => bail!("no container for service {service} in project {}", self.project),
            many => bail!("{} containers for service {service}", many.len()),
        }
    }

    async fn apply(&self, fault: Fault, target: &str) -> Result<()> {
        let id = self.container(target).await?;
        match fault {
            Fault::Kill => {
                let options = KillContainerOptionsBuilder::new().signal("SIGKILL").build();
                self.docker.kill_container(&id, Some(options)).await?;
            }
        }
        Ok(())
    }

    async fn heal(&self, fault: Fault, target: &str) -> Result<()> {
        let id = self.container(target).await?;
        match fault {
            Fault::Kill => self.docker.start_container(&id, None).await?,
        }
        Ok(())
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
                let (fault, target) = (planned.fault, planned.target);
                recorder.record(
                    Proc::Nemesis,
                    Body::FaultStart { fault, target: target.clone() },
                    true,
                )?;
                nemesis
                    .apply(fault, &target)
                    .await
                    .with_context(|| format!("{fault} {target}"))?;
                tokio::select! {
                    _ = stop.cancelled() => {}
                    _ = tokio::time::sleep(planned.heal_after) => {}
                }
                nemesis
                    .heal(fault, &target)
                    .await
                    .with_context(|| format!("heal {fault} {target}"))?;
                recorder.record(Proc::Nemesis, Body::FaultHeal { fault, target }, true)?;
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

    fn spec(targets: &[&str]) -> FaultSpec {
        FaultSpec {
            fault: Fault::Kill,
            target: Targets::Any(targets.iter().map(|target| target.to_string()).collect()),
            at_s: 60,
            heal_after_s: 30,
            repeat_every_s: Some(120),
        }
    }

    #[test]
    fn plan_repeats_until_the_end_of_the_run() {
        let planned = plan(&[spec(&["engine-1"])], 300, 42);
        let starts: Vec<u64> = planned.iter().map(|fault| fault.at.as_secs()).collect();
        assert_eq!(starts, [60, 180]);
        assert!(planned.iter().all(|fault| fault.heal_after == Duration::from_secs(30)));
    }

    #[test]
    fn same_seed_picks_the_same_targets() {
        let specs = [spec(&["engine-1", "engine-2", "engine-3"])];
        let targets = |seed| -> Vec<String> {
            plan(&specs, 3_000, seed).into_iter().map(|fault| fault.target).collect()
        };
        assert_eq!(targets(7), targets(7));
        assert_ne!(targets(7), targets(8));
    }
}
