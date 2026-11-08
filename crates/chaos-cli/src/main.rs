//! `rivet-chaos run <scenario.toml> --seed N` runs one scenario end to end on a
//! fresh cluster: start it, drive load while the nemesis injects faults, stop
//! writes, wait, read every ledger back, let every session finish, collect the
//! sink's records, check, report, archive and tear down.
//!
//! Exit codes: 0 every invariant passed, 2 one failed, 3 one couldn't be
//! checked, 1 the harness itself failed.

use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail};
use chaos_checker::{Settings, Status};
use chaos_core::{
    AlarmRow, EffectRow, Header, HeartbeatRow, SCHEMA_VERSION, Scenario, SinkRecords,
};
use chaos_driver::{Ledgers, LoadConfig, Procs, SessionConfig, Sessions};
use chaos_history::Recorder;
use chaos_nemesis::Nemesis;
use clap::{Parser, Subcommand};
use rivetkit_client::{Client, ClientConfig, EncodingKind};
use serde::{Deserialize, de::DeserializeOwned};
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

mod export;
mod summary;

const PROJECT: &str = "rivet-chaos";
const COMPOSE_FILE: &str = "cluster/compose.yaml";
const ENDPOINT: &str = "http://localhost:6420";
const SINK: &str = "http://localhost:6510";
/// How long every actor gets to answer once before the run starts.
const WARM_UP: Duration = Duration::from_secs(180);
/// After the last session finishes, time for its follow-up alarm to fire.
const ALARM_GRACE: Duration = Duration::from_secs(10);
const SERVICES: [&str; 9] = [
    "engine-1",
    "engine-2",
    "engine-3",
    "haproxy",
    "s3",
    "node-a",
    "node-b",
    "effects-sink",
    "mock-llm",
];

#[derive(Parser)]
#[command(
    name = "rivet-chaos",
    about = "Fault injection and correctness checks for Rivet"
)]
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
        /// Where the run's folder goes.
        #[arg(long, default_value = "results")]
        results: PathBuf,
    },
    /// Check a finished run's history again and rewrite its report.
    Report { run_dir: PathBuf },
    /// Check every run under the given folders again and print one Markdown
    /// table of them, per scenario and per run.
    Summary { dirs: Vec<PathBuf> },
    /// Check every run under the given folders again and write them as JSON for
    /// the dashboard.
    Export {
        dirs: Vec<PathBuf>,
        #[arg(long)]
        out: PathBuf,
    },
}

fn main() -> ExitCode {
    let result = match Cli::parse().command {
        Commands::Run {
            scenario,
            seed,
            keep_cluster,
            results,
        } => tokio::runtime::Runtime::new()
            .map_err(anyhow::Error::from)
            .and_then(|runtime| runtime.block_on(run(&scenario, seed, keep_cluster, &results))),
        Commands::Report { run_dir } => report(&run_dir),
        Commands::Summary { dirs } => summary::print(&dirs).map(|()| Status::Pass),
        Commands::Export { dirs, out } => export::write(&dirs, &out).map(|()| Status::Pass),
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
    shell(
        "gzip",
        &args.iter().map(String::as_str).collect::<Vec<_>>(),
        &[],
    )
}

fn compose(args: &[&str]) -> Command {
    let mut command = Command::new("docker");
    command
        .args([
            "compose",
            "-p",
            PROJECT,
            "--project-directory",
            "cluster",
            "-f",
            "cluster/compose.yaml",
        ])
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
    Some(
        line.trim_start_matches("version = ")
            .trim_matches('"')
            .to_owned(),
    )
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

async fn describe(
    nemesis: &Nemesis,
) -> Result<(BTreeMap<String, String>, BTreeMap<String, String>)> {
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
        format!(
            "{:.1} GiB",
            info.mem_total.unwrap_or_default() as f64 / (1u64 << 30) as f64
        ),
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

async fn run(
    scenario_path: &Path,
    seed: Option<u64>,
    keep_cluster: bool,
    results: &Path,
) -> Result<Status> {
    if !Path::new("cluster/compose.yaml").exists() {
        bail!("run rivet-chaos from the repository root");
    }
    let scenario_toml = fs::read_to_string(scenario_path)
        .with_context(|| format!("read {}", scenario_path.display()))?;
    let scenario = Scenario::parse(&scenario_toml)?;
    let seed = seed.unwrap_or(scenario.seed);
    let run_id = format!("{}-s{seed}-{}", scenario.name, unix_ms() / 1000);
    let dir = results.join(&run_id);
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

    gzip(&[dir.join("history.jsonl"), dir.join("sink.json")])?;
    println!("rivet-chaos: report at {}", dir.join("report.md").display());
    Ok(status)
}

async fn get_json<T: DeserializeOwned>(http: &reqwest::Client, path: &str) -> Result<T> {
    Ok(http
        .get(format!("{SINK}/{path}"))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?)
}

#[derive(Deserialize)]
struct SinkClock {
    mono_ns: u64,
}

/// What to add to a sink reading to put it on the recorder's clock, from the
/// quickest of a few round trips to the sink.
async fn sink_clock_offset(http: &reqwest::Client, recorder: &Recorder) -> Result<i64> {
    let mut best: Option<(u64, i64)> = None;
    for _ in 0..10 {
        let before = recorder.now_ns();
        let clock: SinkClock = get_json(http, "clock").await?;
        let after = recorder.now_ns();
        let offset = ((before + after) / 2) as i64 - clock.mono_ns as i64;
        if best.is_none_or(|(round_trip, _)| after - before < round_trip) {
            best = Some((after - before, offset));
        }
    }
    Ok(best.context("no clock readings")?.1)
}

async fn drive(
    scenario: &Scenario,
    scenario_toml: &str,
    seed: u64,
    run_id: &str,
    dir: &Path,
) -> Result<Status> {
    let nemesis = Nemesis::connect(PROJECT, COMPOSE_FILE, SINK)?;
    let (versions, host) = describe(&nemesis).await?;
    let client = Client::new(
        ClientConfig::new(ENDPOINT)
            .namespace("default")
            .token(read_token()?)
            .encoding(EncodingKind::Json),
    );
    let workload = &scenario.workload;
    let ledgers = Ledgers::new(&client, workload.ledgers)?;
    let sessions = Sessions::new(&client, workload.agent_sessions)?;
    let call_timeout = Duration::from_millis(workload.call_timeout_ms);
    chaos_driver::warm_up(&ledgers, &sessions, call_timeout, WARM_UP).await?;
    println!("rivet-chaos: every actor answered; starting the clock");

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
    let http = reqwest::Client::new();
    let offset_ns = sink_clock_offset(&http, &recorder).await?;
    let procs = Procs::default();

    let started = Instant::now();
    let stop = CancellationToken::new();
    let load = tokio::spawn(chaos_driver::run_load(
        ledgers.clone(),
        recorder.clone(),
        procs.clone(),
        LoadConfig {
            processes: workload.processes,
            pacing_ms: workload.pacing_ms,
            read_percent: workload.read_percent,
            call_timeout,
            seed,
        },
        stop.clone(),
    ));
    let turns = tokio::spawn(chaos_driver::run_sessions(
        sessions.clone(),
        recorder.clone(),
        procs.clone(),
        SessionConfig {
            turn_pacing_ms: workload.turn_pacing_ms,
            call_timeout,
            seed,
        },
        stop.clone(),
    ));
    let plan = chaos_nemesis::plan(&scenario.nemesis, scenario.duration_s, seed);
    println!(
        "rivet-chaos: {} faults planned over {} s",
        plan.len(),
        scenario.duration_s
    );
    let faults = nemesis.run(plan, recorder.clone(), started, CancellationToken::new());

    // Load stops once the duration is over and the last fault has healed.
    let (faults, _) = tokio::join!(
        faults,
        tokio::time::sleep_until(started + Duration::from_secs(scenario.duration_s))
    );
    stop.cancel();
    let load = load.await?;
    let acknowledged = turns.await?;
    faults?;
    load?;
    let acknowledged = acknowledged?;

    println!(
        "rivet-chaos: load stopped, waiting {} s before the final reads",
        scenario.checks.quiesce_s
    );
    tokio::time::sleep(Duration::from_secs(scenario.checks.quiesce_s)).await;
    let deadline = Duration::from_secs(scenario.checks.recovery_deadline_s);
    let (unread, unfinished) = tokio::join!(
        chaos_driver::final_reads(
            ledgers,
            recorder.clone(),
            procs.clone(),
            call_timeout,
            deadline
        ),
        chaos_driver::drain_sessions(
            sessions.clone(),
            recorder.clone(),
            procs,
            acknowledged,
            call_timeout,
            deadline
        ),
    );
    let (unread, unfinished) = (unread?, unfinished?);
    if !unread.is_empty() {
        println!(
            "rivet-chaos: {} ledgers never answered: {}",
            unread.len(),
            unread.join(", ")
        );
    }
    if !unfinished.is_empty() {
        println!(
            "rivet-chaos: {} sessions never finished: {}",
            unfinished.len(),
            unfinished.join(", ")
        );
    }
    recorder.close()?;

    let sink_path = dir.join("sink.json");
    if !sessions.is_empty() {
        tokio::time::sleep(ALARM_GRACE).await;
    }
    let records = SinkRecords {
        offset_ns,
        effects: get_json::<Vec<EffectRow>>(&http, "effects").await?,
        alarms: get_json::<Vec<AlarmRow>>(&http, "alarms").await?,
        heartbeats: get_json::<Vec<HeartbeatRow>>(&http, "heartbeats").await?,
    };
    fs::write(&sink_path, serde_json::to_vec(&records)?)?;

    write_report(dir, &history_path, &sink_path)
}

fn read_maybe_gzipped(path: &Path) -> Result<String> {
    let mut text = String::new();
    let file = File::open(path).with_context(|| format!("open {}", path.display()))?;
    if path.extension().is_some_and(|extension| extension == "gz") {
        flate2::read::GzDecoder::new(file).read_to_string(&mut text)?;
    } else {
        std::io::BufReader::new(file).read_to_string(&mut text)?;
    }
    Ok(text)
}

/// A finished run, read back and checked again.
struct Checked {
    history: chaos_history::History,
    sink: Option<SinkRecords>,
    check: chaos_checker::CheckReport,
}

fn recheck(history_path: &Path, sink_path: &Path) -> Result<Checked> {
    let history = chaos_history::read(history_path)?;
    let sink: Option<SinkRecords> = match sink_path.exists() {
        true => {
            Some(serde_json::from_str(&read_maybe_gzipped(sink_path)?).context("sink records")?)
        }
        false => None,
    };
    let scenario = Scenario::parse(&history.header.scenario_toml)?;
    let settings = Settings {
        recovery_deadline: Duration::from_secs(scenario.checks.recovery_deadline_s),
    };
    let check = chaos_checker::check(&history.events, sink.as_ref(), &settings);
    Ok(Checked {
        history,
        sink,
        check,
    })
}

fn write_report(dir: &Path, history_path: &Path, sink_path: &Path) -> Result<Status> {
    let Checked {
        history,
        sink,
        check,
    } = recheck(history_path, sink_path)?;
    fs::write(
        dir.join("report.md"),
        chaos_report::render(&history, &check, sink.as_ref()),
    )?;
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

/// `name`, or `name.gz`, in `dir`; the plain name if neither exists.
fn archived(dir: &Path, name: &str) -> PathBuf {
    let gzipped = dir.join(format!("{name}.gz"));
    if gzipped.exists() {
        gzipped
    } else {
        dir.join(name)
    }
}

fn report(run_dir: &Path) -> Result<Status> {
    let history = archived(run_dir, "history.jsonl");
    if !history.exists() {
        bail!("no history in {}", run_dir.display());
    }
    let status = write_report(run_dir, &history, &archived(run_dir, "sink.json"))?;
    println!(
        "rivet-chaos: report at {}",
        run_dir.join("report.md").display()
    );
    Ok(status)
}
