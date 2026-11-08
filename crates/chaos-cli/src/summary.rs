//! `rivet-chaos summary <dirs>`: every run under the given folders, checked
//! again from its archived history and sink records, as one Markdown table per
//! scenario and one per run.

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use chaos_checker::Status;
use chaos_core::{Body, Op};

use crate::{Checked, archived, recheck};

const INVARIANTS: [&str; 8] = ["I1", "I2", "I3", "I4", "I5", "I6", "I7", "I8"];

/// The catalogue's order, with the fault each scenario injects.
const SCENARIOS: [(&str, &str); 9] = [
    ("instance-crash", "F1"),
    ("instance-down-long", "F1, held"),
    ("instance-pause", "F2"),
    ("instance-replace", "F3"),
    ("node-crash", "F4"),
    ("node-isolation", "F5"),
    ("instance-isolation", "F6"),
    ("bucket-outage", "F7"),
    ("kill-in-step", "F8"),
];

struct Row {
    run: String,
    scenario: String,
    seed: u64,
    verdict: Status,
    statuses: BTreeMap<&'static str, Status>,
    acknowledged: usize,
    interrupted: usize,
    slowest_ns: i64,
}

fn row(run: String, checked: &Checked) -> Row {
    let Checked { history, check, .. } = checked;
    let acknowledged = history
        .events
        .iter()
        .filter(|event| {
            matches!(
                &event.body,
                Body::Ok {
                    op: Op::Append { .. },
                    ..
                }
            )
        })
        .count();
    Row {
        run,
        scenario: history.header.scenario.clone(),
        seed: history.header.seed,
        verdict: check.verdict(),
        statuses: check
            .invariants
            .iter()
            .map(|result| (result.id, result.status))
            .collect(),
        acknowledged,
        interrupted: check.recoveries.len(),
        slowest_ns: check
            .recoveries
            .iter()
            .map(|recovery| recovery.after_heal_ns)
            .max()
            .unwrap_or(0),
    }
}

/// Run folders: each given folder that holds a history, or else its subfolders that do.
pub(crate) fn run_dirs(dirs: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let has_history = |dir: &Path| archived(dir, "history.jsonl").exists();
    let mut runs = Vec::new();
    for dir in dirs {
        if has_history(dir) {
            runs.push(dir.clone());
            continue;
        }
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() && has_history(&path) {
                runs.push(path);
            }
        }
    }
    runs.sort();
    Ok(runs)
}

fn label(status: Option<&Status>) -> &'static str {
    match status {
        Some(Status::Pass) => "pass",
        Some(Status::Fail) => "**FAIL**",
        Some(Status::Unknown) => "unknown",
        None => "-",
    }
}

fn seconds(ns: i64) -> String {
    format!("{:.1} s", ns as f64 / 1e9)
}

fn order(scenario: &str) -> (usize, &str) {
    let position = SCENARIOS.iter().position(|(name, _)| *name == scenario);
    (position.unwrap_or(SCENARIOS.len()), scenario)
}

pub fn print(dirs: &[PathBuf]) -> Result<()> {
    let mut rows = Vec::new();
    for dir in run_dirs(dirs)? {
        let name = dir
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        match recheck(
            &archived(&dir, "history.jsonl"),
            &archived(&dir, "sink.json"),
        ) {
            Ok(checked) => rows.push(row(name, &checked)),
            Err(error) => eprintln!("rivet-chaos: skipping {}: {error:#}", dir.display()),
        }
    }
    rows.sort_by(|a, b| {
        order(&a.scenario)
            .cmp(&order(&b.scenario))
            .then(a.seed.cmp(&b.seed))
            .then(a.run.cmp(&b.run))
    });

    let mut scenarios: Vec<(&str, Vec<&Row>)> = Vec::new();
    for row in &rows {
        match scenarios.last_mut() {
            Some((name, group)) if *name == row.scenario => group.push(row),
            _ => scenarios.push((&row.scenario, vec![row])),
        }
    }

    println!(
        "| Scenario | Fault | Runs | Passed | {} | Acknowledged appends | Actors interrupted | Slowest recovery after heal |",
        INVARIANTS.join(" | ")
    );
    println!(
        "| --- | --- | ---: | ---: |{} ---: | ---: | ---: |",
        " --- |".repeat(INVARIANTS.len())
    );
    for (scenario, group) in &scenarios {
        let fault = SCENARIOS
            .iter()
            .find(|(name, _)| name == scenario)
            .map_or("", |(_, fault)| fault);
        let passed = group
            .iter()
            .filter(|row| row.verdict == Status::Pass)
            .count();
        let cells: Vec<String> = INVARIANTS
            .iter()
            .map(|id| {
                let statuses: Vec<Status> = group
                    .iter()
                    .filter_map(|row| row.statuses.get(id).copied())
                    .collect();
                let failed = statuses
                    .iter()
                    .filter(|status| **status == Status::Fail)
                    .count();
                if statuses.is_empty() {
                    "-".into()
                } else if failed > 0 {
                    format!("**FAIL** ({failed})")
                } else if statuses.contains(&Status::Unknown) {
                    "unknown".into()
                } else {
                    "pass".into()
                }
            })
            .collect();
        println!(
            "| `{scenario}` | {fault} | {} | {passed} | {} | {} | {} | {} |",
            group.len(),
            cells.join(" | "),
            group.iter().map(|row| row.acknowledged).sum::<usize>(),
            group.iter().map(|row| row.interrupted).sum::<usize>(),
            seconds(group.iter().map(|row| row.slowest_ns).max().unwrap_or(0))
        );
    }

    println!(
        "\n| Run | Seed | {} | Acknowledged appends | Actors interrupted | Slowest recovery after heal |",
        INVARIANTS.join(" | ")
    );
    println!(
        "| --- | ---: |{} ---: | ---: | ---: |",
        " --- |".repeat(INVARIANTS.len())
    );
    for row in &rows {
        let cells: Vec<&str> = INVARIANTS
            .iter()
            .map(|id| label(row.statuses.get(id)))
            .collect();
        println!(
            "| `{}` | {} | {} | {} | {} | {} |",
            row.run,
            row.seed,
            cells.join(" | "),
            row.acknowledged,
            row.interrupted,
            seconds(row.slowest_ns)
        );
    }
    Ok(())
}
