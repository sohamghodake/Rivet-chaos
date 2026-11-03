//! `rivet-chaos run <scenario.toml> --seed N` runs one scenario end to end on a
//! fresh cluster: start it, drive load while the nemesis injects faults, stop
//! writes, wait, read every ledger back, check, report, archive and tear down.
//!
//! Exit codes: 0 every invariant passed, 2 one failed, 3 one couldn't be
//! checked, 1 the harness itself failed.

use std::{
    collections::BTreeMap,
    fs::{self, File},
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail};
use chaos_checker::Status;
use chaos_core::{Header, SCHEMA_VERSION, Scenario};
use chaos_driver::{Ledgers, LoadConfig, Procs};
use chaos_history::Recorder;
use chaos_nemesis::Nemesis;
use clap::{Parser, Subcommand};
use rivetkit_client::{Client, ClientConfig, EncodingKind};
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

const PROJECT: &str = "rivet-chaos";
const ENDPOINT: &str = "http://localhost:6420";
const SERVICES: [&str; 9] = [
    "engine-1", "engine-2", "engine-3", "haproxy", "s3", "node-a", "node-b", "effects-sink", "mock-llm",
];

#[derive(Parser)]
#[command(name = "rivet-chaos", about = "Fault injection and correctness checks for Rivet")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a scenario end to end on a fresh cluster.
    Run {
        scenario: PathBuf,
        /// Overrides the scenario's seed.
        #[arg(long)]
        seed: Option<u64>,
        /// Leave the cluster running afterwards, for poking at.
        #[arg(long)]
        keep_cluster: bool,
    },
    /// Check a finished run's history again and rewrite its report.
    Report { run_dir: PathBuf },
}

fn main() -> ExitCode {
    let result = match Cli::parse().command {
        Commands::Run { scenario, seed, keep_cluster } => tokio::runtime::Runtime::new()
            .map_err(anyhow::Error::from)
            .and_then(|runtime| runtime.block_on(run(&scenario, seed, keep_cluster))),
        Commands::Report { run_dir } => report(&run_dir),
    };
    match result {
        Ok(Status::Pass) => ExitCode::SUCCESS,
        Ok(Status::Fail) => ExitCode::from(2),
        Ok(Status::Unknown) => ExitCode::from(3),
        Err(error) => {
            eprintln!("rivet-chaos: {error:#}");
            ExitCode::FAILURE
        }
    }
}

fn shell(program: &str, args: &[&str], envs: &[(&str, String)]) -> Result<()> {
    let status = Command::new(program)
        .args(args)
        .envs(envs.iter().map(|(key, value)| (*key, value.as_str())))
        .status()
        .with_context(|| format!("run {program}"))?;
    if !status.success() {
        bail!("{program} {} exited with {status}", args.join(" "));
    }
    Ok(())
}

fn gzip(paths: &[PathBuf]) -> Result<()> {
    let mut args = vec!["--force".to_owned()];
    args.extend(paths.iter().map(|path| path.to_string_lossy().into_owned()));
    shell("gzip", &args.iter().map(String::as_str).collect::<Vec<_>>(), &[])
}

fn compose(args: &[&str]) -> Command {
    let mut command = Command::new("docker");
    command
        .args(["compose", "-p", PROJECT, "--project-directory", "cluster", "-f", "cluster/compose.yaml"])
        .args(args);
    command
}

fn unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock is before 1970")
        .as_millis() as u64
}

fn read_token() -> Result<String> {
    let env = fs::read_to_string("cluster/.env").context("read cluster/.env")?;
    env.lines()
        .find_map(|line| line.strip_prefix("RIVET_TOKEN="))
        .map(str::to_owned)
        .context("cluster/.env has no RIVET_TOKEN")
}

/// The first `version = "..."` after `name = "<package>"` in Cargo.lock.
fn locked_version(package: &str) -> Option<String> {
    let lock = fs::read_to_string("Cargo.lock").ok()?;
    let start = lock.find(&format!("name = \"{package}\"\n"))?;
    let rest = &lock[start..];
    let line = rest.lines().find(|line| line.starts_with("version = "))?;
    Some(line.trim_start_matches("version = ").trim_matches('"').to_owned())
}

fn git_commit() -> String {
    Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_else(|| "none".into())
}

async fn describe(nemesis: &Nemesis) -> Result<(BTreeMap<String, String>, BTreeMap<String, String>)> {
    let mut versions = BTreeMap::new();
    let engine = nemesis.container("engine-1").await?;
    let image = nemesis
        .docker()
        .inspect_container(&engine, None)
        .await?
        .config
        .and_then(|config| config.image)
        .unwrap_or_default();
    versions.insert("Engine image".into(), format!("`{image}`"));
    let workload: serde_json::Value =
        serde_json::from_str(&fs::read_to_string("workload/package.json")?)?;
    if let Some(rivetkit) = workload["dependencies"]["rivetkit"].as_str() {
        versions.insert("rivetkit".into(), rivetkit.into());
    }
    if let Some(client) = locked_version("rivetkit-client") {
        versions.insert("rivetkit-client".into(), client);
    }
    versions.insert("rivet-chaos commit".into(), git_commit());

    let info = nemesis.docker().info().await?;
    let mut host = BTreeMap::new();
    host.insert("CPUs".into(), info.ncpu.unwrap_or_default().to_string());
    host.insert(
        "Memory".into(),
        format!("{:.1} GiB", info.mem_total.unwrap_or_default() as f64 / (1u64 << 30) as f64),
    );
    host.insert(
        "Docker".into(),
        format!(
            "{} on {}, kernel {}",
            info.server_version.unwrap_or_default(),
            info.operating_system.unwrap_or_default(),
            info.kernel_version.unwrap_or_default()
        ),
    );
    Ok((versions, host))
}

async fn run(scenario_path: &Path, seed: Option<u64>, keep_cluster: bool) -> Result<Status> {
    if !Path::new("cluster/compose.yaml").exists() {
        bail!("run rivet-chaos from the repository root");
    }
    let scenario_toml = fs::read_to_string(scenario_path)
        .with_context(|| format!("read {}", scenario_path.display()))?;
    let scenario = Scenario::parse(&scenario_toml)?;
    let seed = seed.unwrap_or(scenario.seed);
    let run_id = format!("{}-s{seed}-{}", scenario.name, unix_ms() / 1000);
    let dir = PathBuf::from("results").join(&run_id);
    fs::create_dir_all(dir.join("logs"))?;
    println!("rivet-chaos: run {run_id}");

    shell("cluster/down.sh", &[], &[])?;
    shell("cluster/up.sh", &[], &[("SEED", seed.to_string())])?;

    let outcome = drive(&scenario, &scenario_toml, seed, &run_id, &dir).await;

    // Logs and teardown happen whether or not the run itself worked.
    let mut logs = Vec::new();
    for service in SERVICES {
        let path = dir.join("logs").join(format!("{service}.log"));
        let log = File::create(&path)?;
        let _ = compose(&["logs", "--no-color", "--timestamps", service])
            .stdout(log.try_clone()?)
            .stderr(log)
            .status();
        logs.push(path);
    }
    if !keep_cluster {
        shell("cluster/down.sh", &[], &[])?;
    }
    // Gzipped, a run's history and logs are small enough to keep in git.
    gzip(&logs)?;
    let status = outcome?;

    gzip(&[dir.join("history.jsonl")])?;
    println!("rivet-chaos: report at {}", dir.join("report.md").display());
    Ok(status)
}

async fn drive(scenario: &Scenario, scenario_toml: &str, seed: u64, run_id: &str, dir: &Path) -> Result<Status> {
    let nemesis = Nemesis::connect(PROJECT)?;
    let (versions, host) = describe(&nemesis).await?;
    let header = Header {
        schema: SCHEMA_VERSION,
        run: run_id.into(),
        scenario: scenario.name.clone(),
        seed,
        started_wall_ms: unix_ms(),
        scenario_toml: scenario_toml.into(),
        versions,
        host,
    };
    let history_path = dir.join("history.jsonl");
    let recorder = Recorder::create(&history_path, &header)?;

    let client = Client::new(
        ClientConfig::new(ENDPOINT)
            .namespace("default")
            .token(read_token()?)
            .encoding(EncodingKind::Json),
    );
    let ledgers = Ledgers::new(&client, scenario.workload.ledgers)?;
    let procs = Procs::default();
    let call_timeout = Duration::from_millis(scenario.workload.call_timeout_ms);

    let started = Instant::now();
    let stop = CancellationToken::new();
    let load = tokio::spawn(chaos_driver::run_load(
        ledgers.clone(),
        recorder.clone(),
        procs.clone(),
        LoadConfig {
            processes: scenario.workload.processes,
            pacing_ms: scenario.workload.pacing_ms,
            call_timeout,
            seed,
        },
        stop.clone(),
    ));
    let plan = chaos_nemesis::plan(&scenario.nemesis, scenario.duration_s, seed);
    println!("rivet-chaos: {} faults planned over {} s", plan.len(), scenario.duration_s);
    let faults = nemesis.run(plan, recorder.clone(), started, CancellationToken::new());

    // Writes stop once the duration is over and the last fault has healed.
    let (faults, _) = tokio::join!(faults, tokio::time::sleep_until(started + Duration::from_secs(scenario.duration_s)));
    stop.cancel();
    let load = load.await?;
    faults?;
    load?;

    println!("rivet-chaos: load stopped, waiting {} s before the final reads", scenario.checks.quiesce_s);
    tokio::time::sleep(Duration::from_secs(scenario.checks.quiesce_s)).await;
    let unread = chaos_driver::final_reads(
        ledgers,
        recorder.clone(),
        procs,
        call_timeout,
        Duration::from_secs(scenario.checks.recovery_deadline_s),
    )
    .await?;
    if !unread.is_empty() {
        println!("rivet-chaos: {} ledgers never answered: {}", unread.len(), unread.join(", "));
    }
    recorder.close()?;

    write_report(dir, &history_path)
}

fn write_report(dir: &Path, history_path: &Path) -> Result<Status> {
    let history = chaos_history::read(history_path)?;
    let check = chaos_checker::check(&history.events);
    fs::write(dir.join("report.md"), chaos_report::render(&history, &check))?;
    for result in &check.invariants {
        println!(
            "rivet-chaos: {} {}: {:?} ({} checked, {} violations)",
            result.id,
            result.name,
            result.status,
            result.checked,
            result.violations.len()
        );
    }
    Ok(check.verdict())
}

fn report(run_dir: &Path) -> Result<Status> {
    let history = ["history.jsonl", "history.jsonl.gz"]
        .map(|name| run_dir.join(name))
        .into_iter()
        .find(|path| path.exists())
        .with_context(|| format!("no history in {}", run_dir.display()))?;
    let status = write_report(run_dir, &history)?;
    println!("rivet-chaos: report at {}", run_dir.join("report.md").display());
    Ok(status)
}
