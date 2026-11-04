//! Markdown report for one run, in the shape of Rivet's benchmarking page: p50
//! and p99 with sample counts, hardware and versions, and every failure.

use std::{collections::HashMap, fmt::Write};

use chaos_checker::{CheckReport, Status};
use chaos_core::{Body, Event, Fault, Network, Op, Proc, SinkRecords};
use chaos_history::History;
use hdrhistogram::Histogram;

/// Violations listed per invariant before the rest are summarized.
const MAX_LISTED: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Window {
    Before,
    During,
    After,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Ok,
    Fail,
    Info,
    Pending,
}

struct Call<'a> {
    op: &'a Op,
    invoked_ns: u64,
    done_ns: Option<u64>,
    outcome: Outcome,
    error: Option<&'a str>,
}

struct FaultWindow<'a> {
    fault: Fault,
    target: &'a str,
    network: Option<Network>,
    start_ns: u64,
    heal_ns: Option<u64>,
}

fn calls(events: &[Event]) -> Vec<Call<'_>> {
    let mut open: HashMap<Proc, usize> = HashMap::new();
    let mut calls = Vec::new();
    for event in events {
        let (outcome, error) = match &event.body {
            Body::Invoke { op } => {
                open.insert(event.proc, calls.len());
                calls.push(Call {
                    op,
                    invoked_ns: event.mono_ns,
                    done_ns: None,
                    outcome: Outcome::Pending,
                    error: None,
                });
                continue;
            }
            Body::Ok { .. } => (Outcome::Ok, None),
            Body::Fail { error, .. } => (Outcome::Fail, Some(error.as_str())),
            Body::Info { error, .. } => (Outcome::Info, Some(error.as_str())),
            Body::FaultStart { .. } | Body::FaultHeal { .. } => continue,
        };
        if let Some(index) = open.remove(&event.proc) {
            let call = &mut calls[index];
            call.done_ns = Some(event.mono_ns);
            call.outcome = outcome;
            call.error = error;
        }
    }
    calls
}

fn fault_windows(events: &[Event]) -> Vec<FaultWindow<'_>> {
    let mut windows: Vec<FaultWindow> = Vec::new();
    for event in events {
        match &event.body {
            Body::FaultStart { fault, target, network } => windows.push(FaultWindow {
                fault: *fault,
                target,
                network: *network,
                start_ns: event.mono_ns,
                heal_ns: None,
            }),
            Body::FaultHeal { fault, target, .. } => {
                if let Some(window) = windows.iter_mut().find(|window| {
                    window.heal_ns.is_none() && window.fault == *fault && window.target == target
                }) {
                    window.heal_ns = Some(event.mono_ns);
                }
            }
            _ => {}
        }
    }
    windows
}

fn window_of(invoked_ns: u64, faults: &[FaultWindow]) -> Window {
    match faults.first() {
        None => Window::Before,
        Some(first) if invoked_ns < first.start_ns => Window::Before,
        _ if faults.iter().any(|fault| {
            invoked_ns >= fault.start_ns && fault.heal_ns.is_none_or(|heal| invoked_ns <= heal)
        }) =>
        {
            Window::During
        }
        _ => Window::After,
    }
}

fn seconds(ns: u64) -> String {
    format!("{:.1} s", ns as f64 / 1e9)
}

fn millis(micros: u64) -> String {
    format!("{:.1} ms", micros as f64 / 1000.0)
}

fn signed_seconds(ns: i64) -> String {
    format!("{:.1} s", ns as f64 / 1e9)
}

fn percentile(sorted: &[i64], percent: usize) -> i64 {
    sorted[(sorted.len() - 1) * percent / 100]
}

fn status_label(status: Status) -> &'static str {
    match status {
        Status::Pass => "PASS",
        Status::Fail => "FAIL",
        Status::Unknown => "UNKNOWN",
    }
}

pub fn render(history: &History, check: &CheckReport, sink: Option<&SinkRecords>) -> String {
    let header = &history.header;
    let calls = calls(&history.events);
    let faults = fault_windows(&history.events);
    let mut out = String::new();

    let _ = writeln!(out, "# {}\n", header.run);
    let _ = writeln!(out, "**Verdict: {}**\n", status_label(check.verdict()));

    let started = jiff::Timestamp::from_millisecond(header.started_wall_ms as i64)
        .map(|time| time.strftime("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|_| header.started_wall_ms.to_string());
    let _ = writeln!(out, "| | |\n| --- | --- |");
    let _ = writeln!(out, "| Scenario | `{}` |", header.scenario);
    let _ = writeln!(out, "| Seed | {} |", header.seed);
    let _ = writeln!(out, "| Started | {started} |");
    for (key, value) in header.versions.iter().chain(&header.host) {
        let _ = writeln!(out, "| {key} | {value} |");
    }

    let _ = writeln!(out, "\n## Invariants\n");
    let _ = writeln!(out, "| ID | Invariant | Result | Checked | Violations |");
    let _ = writeln!(out, "| --- | --- | --- | ---: | ---: |");
    for result in &check.invariants {
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | {} |",
            result.id,
            result.name,
            status_label(result.status),
            result.checked,
            result.violations.len()
        );
    }
    if !check.unreadable.is_empty() {
        let _ = writeln!(
            out,
            "\n{} ledgers had no successful read after their last append, so they were not checked: {}",
            check.unreadable.len(),
            check.unreadable.join(", ")
        );
    }
    for result in check.invariants.iter().filter(|result| !result.notes.is_empty()) {
        for note in &result.notes {
            let _ = writeln!(out, "\n{}: {note}", result.id);
        }
    }
    for result in check.invariants.iter().filter(|result| !result.violations.is_empty()) {
        let _ = writeln!(out, "\n### {} violations\n", result.id);
        for violation in result.violations.iter().take(MAX_LISTED) {
            let _ = writeln!(out, "- {violation}");
        }
        if result.violations.len() > MAX_LISTED {
            let _ = writeln!(out, "- … and {} more", result.violations.len() - MAX_LISTED);
        }
    }

    let _ = writeln!(out, "\n## Faults\n");
    if faults.is_empty() {
        let _ = writeln!(out, "None.");
    } else {
        let _ = writeln!(out, "| # | Fault | Target | Start | Heal | Length |");
        let _ = writeln!(out, "| ---: | --- | --- | ---: | ---: | ---: |");
        for (index, fault) in faults.iter().enumerate() {
            let (heal, length) = match fault.heal_ns {
                Some(heal) => (seconds(heal), seconds(heal - fault.start_ns)),
                None => ("never".into(), "-".into()),
            };
            let name = match fault.network {
                Some(network) => format!("{} from `{network}`", fault.fault),
                None => fault.fault.to_string(),
            };
            let _ = writeln!(
                out,
                "| {} | {name} | {} | {} | {heal} | {length} |",
                index + 1,
                fault.target,
                seconds(fault.start_ns)
            );
        }
    }

    if !faults.is_empty() {
        let _ = writeln!(out, "\n## Recovery\n");
        let _ = writeln!(
            out,
            "Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.\n"
        );
        let _ = writeln!(out, "| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |");
        let _ = writeln!(out, "| ---: | ---: | ---: | ---: | ---: | ---: |");
        for index in 0..faults.len() {
            let recoveries: Vec<_> = check.recoveries.iter().filter(|recovery| recovery.fault == index).collect();
            if recoveries.is_empty() {
                let _ = writeln!(out, "| {} | 0 | - | - | - | - |", index + 1);
                continue;
            }
            let mut after: Vec<i64> = recoveries.iter().map(|recovery| recovery.after_heal_ns).collect();
            after.sort_unstable();
            let early = after.iter().filter(|ns| **ns < 0).count();
            let outage = recoveries.iter().map(|recovery| recovery.outage_ns).max().unwrap_or(0);
            let _ = writeln!(
                out,
                "| {} | {} | {early} | {} | {} | {} |",
                index + 1,
                recoveries.len(),
                signed_seconds(percentile(&after, 50)),
                signed_seconds(*after.last().unwrap()),
                seconds(outage)
            );
        }
    }

    let _ = writeln!(out, "\n## Appends\n");
    let _ = writeln!(
        out,
        "Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.\n"
    );
    let _ = writeln!(out, "| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |");
    let _ = writeln!(out, "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |");
    let appends: Vec<&Call> = calls.iter().filter(|call| matches!(call.op, Op::Append { .. })).collect();
    for (label, window) in [
        ("Before faults", Some(Window::Before)),
        ("During faults", Some(Window::During)),
        ("After faults", Some(Window::After)),
        ("All", None),
    ] {
        let selected: Vec<&&Call> = appends
            .iter()
            .filter(|call| window.is_none_or(|window| window_of(call.invoked_ns, &faults) == window))
            .collect();
        if selected.is_empty() && window.is_some() {
            continue;
        }
        let count = |outcome| selected.iter().filter(|call| call.outcome == outcome).count();
        let ok = count(Outcome::Ok);
        let mut histogram = Histogram::<u64>::new(3).expect("valid precision");
        for call in selected.iter().filter(|call| call.outcome == Outcome::Ok) {
            let micros = (call.done_ns.unwrap_or(call.invoked_ns) - call.invoked_ns) / 1000;
            histogram.record(micros.max(1)).expect("auto-resizing histogram");
        }
        let (p50, p99, max) = if ok == 0 {
            ("-".into(), "-".into(), "-".into())
        } else {
            (
                millis(histogram.value_at_quantile(0.5)),
                millis(histogram.value_at_quantile(0.99)),
                millis(histogram.max()),
            )
        };
        let _ = writeln!(
            out,
            "| {label} | {} | {ok} | {} | {} | {:.2}% | {p50} | {p99} | {max} |",
            selected.len(),
            count(Outcome::Fail),
            count(Outcome::Info) + count(Outcome::Pending),
            100.0 * ok as f64 / selected.len().max(1) as f64
        );
    }

    // Reads issued once the load had stopped are the final reads.
    let last_append = appends.iter().map(|call| call.invoked_ns).max().unwrap_or(0);
    let (final_reads, load_reads): (Vec<&Call>, Vec<&Call>) = calls
        .iter()
        .filter(|call| matches!(call.op, Op::Read { .. }))
        .partition(|call| call.invoked_ns > last_append);
    if !load_reads.is_empty() {
        let ok = load_reads.iter().filter(|call| call.outcome == Outcome::Ok).count();
        let _ = writeln!(out, "\nReads during the load: {ok} succeeded out of {}.", load_reads.len());
    }
    if !final_reads.is_empty() {
        let ok = final_reads.iter().filter(|call| call.outcome == Outcome::Ok).count();
        let first = final_reads.iter().map(|call| call.invoked_ns).min().unwrap_or(0);
        let last = final_reads.iter().filter_map(|call| call.done_ns).max().unwrap_or(first);
        let _ = writeln!(
            out,
            "\nFinal reads: {ok} succeeded out of {} attempts, over {}.",
            final_reads.len(),
            seconds(last - first)
        );
    }

    let sends: Vec<&Call> = calls.iter().filter(|call| matches!(call.op, Op::Send { .. })).collect();
    if !sends.is_empty() {
        let sessions: std::collections::HashSet<&str> = sends.iter().map(|call| call.op.actor()).collect();
        let ok = sends.iter().filter(|call| call.outcome == Outcome::Ok).count();
        let info = sends.iter().filter(|call| matches!(call.outcome, Outcome::Info | Outcome::Pending)).count();
        let _ = writeln!(out, "\n## Sessions\n");
        let _ = writeln!(
            out,
            "{} sessions were sent {} turns: {ok} acknowledged, {info} with an unknown outcome, {} failed.",
            sessions.len(),
            sends.len(),
            sends.len() - ok - info
        );
        if let Some(sink) = sink {
            let keys: std::collections::HashSet<&str> = sink.effects.iter().map(|row| row.key.as_str()).collect();
            let _ = writeln!(
                out,
                "\nThe sink recorded {} tool calls for {} distinct steps.",
                sink.effects.len(),
                keys.len()
            );
            let mut delays: Vec<i64> = sink
                .alarms
                .iter()
                .map(|alarm| alarm.mono_ns as i64 - alarm.due_mono_ns as i64)
                .collect();
            delays.sort_unstable();
            if !delays.is_empty() {
                let _ = writeln!(
                    out,
                    "\n{} alarms fired, after their due time by p50 {} and at most {}.",
                    delays.len(),
                    millis((percentile(&delays, 50).max(0) / 1000) as u64),
                    millis(((*delays.last().unwrap()).max(0) / 1000) as u64)
                );
            }
        }
    }

    let mut errors: HashMap<&str, usize> = HashMap::new();
    for call in &calls {
        if let Some(error) = call.error {
            *errors.entry(error).or_default() += 1;
        }
    }
    if !errors.is_empty() {
        let mut errors: Vec<(&str, usize)> = errors.into_iter().collect();
        errors.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
        let _ = writeln!(out, "\n## Errors\n");
        let _ = writeln!(out, "| Count | Error |\n| ---: | --- |");
        for (error, count) in errors {
            let error: String = error.chars().take(200).collect();
            let _ = writeln!(out, "| {count} | `{}` |", error.replace('|', "\\|").replace('\n', " "));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use chaos_core::{Header, SCHEMA_VERSION};

    use super::*;

    fn event(seq: u64, proc: Proc, body: Body, mono_ms: u64) -> Event {
        Event {
            run: "r".into(),
            seq,
            proc,
            body,
            mono_ns: mono_ms * 1_000_000,
            wall_ms: None,
        }
    }

    fn append(n: i64) -> Op {
        Op::Append {
            actor: "ledger/0".into(),
            op_id: format!("p0-{n:06}"),
            value: n,
        }
    }

    #[test]
    fn appends_are_split_around_the_fault() {
        let ok = |op| Body::Ok { op, op_ids: None, turns: None, duplicate: false };
        let events = vec![
            event(1, Proc::Client(0), Body::Invoke { op: append(1) }, 0),
            event(2, Proc::Client(0), ok(append(1)), 10),
            event(3, Proc::Nemesis, Body::FaultStart { fault: Fault::Kill, target: "engine-1".into(), network: None }, 100),
            event(4, Proc::Client(0), Body::Invoke { op: append(2) }, 110),
            event(5, Proc::Client(0), Body::Info { op: append(2), error: "timeout".into() }, 5110),
            event(6, Proc::Nemesis, Body::FaultHeal { fault: Fault::Kill, target: "engine-1".into(), network: None }, 6000),
            event(7, Proc::Client(1), Body::Invoke { op: append(3) }, 7000),
            event(8, Proc::Client(1), ok(append(3)), 7030),
        ];
        let history = History {
            header: Header {
                schema: SCHEMA_VERSION,
                run: "r".into(),
                scenario: "s".into(),
                seed: 1,
                started_wall_ms: 1_793_721_600_000,
                scenario_toml: String::new(),
                versions: BTreeMap::from([("engine".into(), "3.0.0".into())]),
                host: BTreeMap::new(),
            },
            events,
        };
        let report = render(&history, &chaos_checker::check(&history.events, None, &Default::default()), None);

        assert!(report.contains("| Before faults | 1 | 1 | 0 | 0 | 100.00% | 10.0 ms |"), "{report}");
        assert!(report.contains("| During faults | 1 | 0 | 0 | 1 | 0.00% | - |"), "{report}");
        assert!(report.contains("| After faults | 1 | 1 | 0 | 0 | 100.00% | 30.0 ms |"), "{report}");
        assert!(report.contains("| 1 | kill | engine-1 | 0.1 s | 6.0 s | 5.9 s |"), "{report}");
        assert!(report.contains("| 1 | `timeout` |"), "{report}");
        assert!(report.contains("| Started | 2026-11-03 16:00:00 UTC |"), "{report}");
        assert!(report.contains("| engine | 3.0.0 |"), "{report}");
        // engine-1's only failed call was at 0.11 s; it answered again at 7.03 s, 1.0 s after the heal.
        assert!(report.contains("| 1 | 1 | 0 | 1.0 s | 1.0 s | 6.9 s |"), "{report}");
    }
}
