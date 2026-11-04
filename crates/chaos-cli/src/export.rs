//! `rivet-chaos export <dirs> --out <dir>`: every run under the given folders,
//! checked again, as compact JSON for the dashboard: `index.json` lists the runs,
//! and `runs/<run>.json` holds each one's timeline, faults and recoveries.

use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use chaos_checker::Status;
use chaos_core::{Body, Event, Fault, Network, Op, Proc};
use serde::Serialize;

use crate::{Checked, archived, recheck, summary::run_dirs};

/// Violations kept per invariant; the report has the rest.
const VIOLATIONS_KEPT: usize = 20;

#[derive(Serialize)]
struct IndexEntry {
    run: String,
    /// The folder the run is in, such as `phase2` or `phase3`.
    group: String,
    scenario: String,
    seed: u64,
    started_wall_ms: u64,
    verdict: &'static str,
    statuses: BTreeMap<&'static str, &'static str>,
    acknowledged: usize,
    faults: usize,
    interrupted: usize,
    slowest_after_heal_s: f64,
}

#[derive(Serialize)]
struct RunExport {
    run: String,
    group: String,
    scenario: String,
    seed: u64,
    started_wall_ms: u64,
    duration_s: f64,
    verdict: &'static str,
    versions: BTreeMap<String, String>,
    host: BTreeMap<String, String>,
    invariants: Vec<InvariantExport>,
    faults: Vec<FaultExport>,
    seconds: Vec<Second>,
    recoveries: Vec<RecoveryExport>,
    totals: Totals,
}

#[derive(Serialize)]
struct InvariantExport {
    id: &'static str,
    name: &'static str,
    status: &'static str,
    checked: usize,
    violation_count: usize,
    violations: Vec<String>,
    notes: Vec<String>,
}

#[derive(Serialize)]
struct FaultExport {
    fault: Fault,
    target: String,
    network: Option<Network>,
    start_s: f64,
    heal_s: Option<f64>,
}

/// Ledger appends invoked during one second of the run.
#[derive(Serialize, Default)]
struct Second {
    t: u64,
    acknowledged: usize,
    /// Ended in `info` or never finished: possibly applied.
    unknown: usize,
    failed: usize,
    p50_ms: Option<f64>,
    p99_ms: Option<f64>,
    max_ms: Option<f64>,
}

#[derive(Serialize)]
struct RecoveryExport {
    fault: usize,
    actor: String,
    failed_s: f64,
    answered_s: f64,
    outage_s: f64,
    after_heal_s: f64,
}

#[derive(Serialize, Default)]
struct Totals {
    appends: usize,
    acknowledged: usize,
    unknown: usize,
    failed: usize,
    reads: usize,
    turns_sent: usize,
    turns_acknowledged: usize,
}

fn status_label(status: Status) -> &'static str {
    match status {
        Status::Pass => "pass",
        Status::Fail => "fail",
        Status::Unknown => "unknown",
    }
}

fn secs(ns: impl Into<i128>) -> f64 {
    millis_round(ns.into() as f64 / 1e9)
}

/// Seconds to the millisecond, so the JSON carries no float noise.
fn millis_round(seconds: f64) -> f64 {
    (seconds * 1e3).round() / 1e3
}

/// The value at `percent` of a sorted list, by nearest rank.
fn percentile(sorted: &[f64], percent: usize) -> f64 {
    sorted[((sorted.len() * percent).div_ceil(100)).saturating_sub(1)]
}

#[derive(Clone, Copy, PartialEq)]
enum Outcome {
    Pending,
    Ok,
    Fail,
    Info,
}

struct Call<'a> {
    op: &'a Op,
    invoke_ns: u64,
    done_ns: Option<u64>,
    outcome: Outcome,
}

fn calls(events: &[Event]) -> Vec<Call<'_>> {
    let mut open: HashMap<Proc, usize> = HashMap::new();
    let mut calls = Vec::new();
    for event in events {
        let outcome = match &event.body {
            Body::Invoke { op } => {
                open.insert(event.proc, calls.len());
                calls.push(Call {
                    op,
                    invoke_ns: event.mono_ns,
                    done_ns: None,
                    outcome: Outcome::Pending,
                });
                continue;
            }
            Body::Ok { .. } => Outcome::Ok,
            Body::Fail { .. } => Outcome::Fail,
            Body::Info { .. } => Outcome::Info,
            Body::FaultStart { .. } | Body::FaultHeal { .. } => continue,
        };
        if let Some(index) = open.remove(&event.proc) {
            calls[index].done_ns = Some(event.mono_ns);
            calls[index].outcome = outcome;
        }
    }
    calls
}

fn faults(events: &[Event]) -> Vec<FaultExport> {
    let mut faults: Vec<FaultExport> = Vec::new();
    for event in events {
        match &event.body {
            Body::FaultStart { fault, target, network } => faults.push(FaultExport {
                fault: *fault,
                target: target.clone(),
                network: *network,
                start_s: secs(event.mono_ns),
                heal_s: None,
            }),
            Body::FaultHeal { fault, target, .. } => {
                if let Some(open) = faults
                    .iter_mut()
                    .find(|open| open.heal_s.is_none() && open.fault == *fault && open.target == *target)
                {
                    open.heal_s = Some(secs(event.mono_ns));
                }
            }
            _ => {}
        }
    }
    faults
}

fn seconds(calls: &[Call]) -> Vec<Second> {
    let mut buckets: BTreeMap<u64, (Second, Vec<f64>)> = BTreeMap::new();
    for call in calls.iter().filter(|call| matches!(call.op, Op::Append { .. })) {
        let t = call.invoke_ns / 1_000_000_000;
        let (second, latencies) = buckets.entry(t).or_insert_with(|| (Second { t, ..Default::default() }, Vec::new()));
        match call.outcome {
            Outcome::Ok => {
                second.acknowledged += 1;
                if let Some(done_ns) = call.done_ns {
                    latencies.push((done_ns - call.invoke_ns) as f64 / 1e6);
                }
            }
            Outcome::Info | Outcome::Pending => second.unknown += 1,
            Outcome::Fail => second.failed += 1,
        }
    }
    let last = buckets.keys().last().copied().unwrap_or(0);
    (0..=last)
        .map(|t| match buckets.remove(&t) {
            None => Second { t, ..Default::default() },
            Some((mut second, mut latencies)) => {
                latencies.sort_by(f64::total_cmp);
                if !latencies.is_empty() {
                    let round = |ms: f64| (ms * 100.0).round() / 100.0;
                    second.p50_ms = Some(round(percentile(&latencies, 50)));
                    second.p99_ms = Some(round(percentile(&latencies, 99)));
                    second.max_ms = Some(round(*latencies.last().unwrap()));
                }
                second
            }
        })
        .collect()
}

fn totals(calls: &[Call]) -> Totals {
    let mut totals = Totals::default();
    for call in calls {
        match call.op {
            Op::Append { .. } => {
                totals.appends += 1;
                match call.outcome {
                    Outcome::Ok => totals.acknowledged += 1,
                    Outcome::Info | Outcome::Pending => totals.unknown += 1,
                    Outcome::Fail => totals.failed += 1,
                }
            }
            Op::Read { .. } => totals.reads += 1,
            Op::Send { .. } => {
                totals.turns_sent += 1;
                totals.turns_acknowledged += usize::from(call.outcome == Outcome::Ok);
            }
            Op::Turns { .. } => {}
        }
    }
    totals
}

fn export_run(dir: &Path, checked: &Checked) -> (IndexEntry, RunExport) {
    let Checked { history, check, .. } = checked;
    let header = &history.header;
    let calls = calls(&history.events);
    let faults = faults(&history.events);
    let recoveries: Vec<RecoveryExport> = check
        .recoveries
        .iter()
        .filter_map(|recovery| {
            let heal_s = faults.get(recovery.fault)?.heal_s?;
            let answered_s = millis_round(heal_s + secs(recovery.after_heal_ns));
            Some(RecoveryExport {
                fault: recovery.fault,
                actor: recovery.actor.clone(),
                failed_s: millis_round(answered_s - secs(recovery.outage_ns)),
                answered_s,
                outage_s: secs(recovery.outage_ns),
                after_heal_s: secs(recovery.after_heal_ns),
            })
        })
        .collect();
    let totals = totals(&calls);
    let run = dir.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default();
    let group = dir
        .parent()
        .and_then(Path::file_name)
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    let verdict = status_label(check.verdict());
    let entry = IndexEntry {
        run: run.clone(),
        group: group.clone(),
        scenario: header.scenario.clone(),
        seed: header.seed,
        started_wall_ms: header.started_wall_ms,
        verdict,
        statuses: check.invariants.iter().map(|result| (result.id, status_label(result.status))).collect(),
        acknowledged: totals.acknowledged,
        faults: faults.len(),
        interrupted: recoveries.len(),
        slowest_after_heal_s: recoveries.iter().map(|recovery| recovery.after_heal_s).fold(0.0, f64::max),
    };
    let export = RunExport {
        run,
        group,
        scenario: header.scenario.clone(),
        seed: header.seed,
        started_wall_ms: header.started_wall_ms,
        duration_s: secs(history.events.last().map_or(0, |event| event.mono_ns)),
        verdict,
        versions: header.versions.clone(),
        host: header.host.clone(),
        invariants: check
            .invariants
            .iter()
            .map(|result| InvariantExport {
                id: result.id,
                name: result.name,
                status: status_label(result.status),
                checked: result.checked,
                violation_count: result.violations.len(),
                violations: result.violations.iter().take(VIOLATIONS_KEPT).cloned().collect(),
                notes: result.notes.clone(),
            })
            .collect(),
        faults,
        seconds: seconds(&calls),
        recoveries,
        totals,
    };
    (entry, export)
}

pub fn write(dirs: &[PathBuf], out: &Path) -> Result<()> {
    let runs_dir = out.join("runs");
    fs::create_dir_all(&runs_dir).with_context(|| format!("create {}", runs_dir.display()))?;
    let mut index = Vec::new();
    for dir in run_dirs(dirs)? {
        match recheck(&archived(&dir, "history.jsonl"), &archived(&dir, "sink.json")) {
            Ok(checked) => {
                let (entry, export) = export_run(&dir, &checked);
                fs::write(runs_dir.join(format!("{}.json", entry.run)), serde_json::to_vec(&export)?)?;
                index.push(entry);
            }
            Err(error) => eprintln!("rivet-chaos: skipping {}: {error:#}", dir.display()),
        }
    }
    index.sort_by_key(|entry| entry.started_wall_ms);
    fs::write(out.join("index.json"), serde_json::to_vec(&index)?)?;
    println!("rivet-chaos: exported {} runs to {}", index.len(), out.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentile_is_nearest_rank() {
        let values: Vec<f64> = (1..=100).map(f64::from).collect();
        assert_eq!(percentile(&values, 50), 50.0);
        assert_eq!(percentile(&values, 99), 99.0);
        assert_eq!(percentile(&[7.0], 99), 7.0);
        assert_eq!(percentile(&[1.0, 2.0, 3.0], 50), 2.0);
    }
}
