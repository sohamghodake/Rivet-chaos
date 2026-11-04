//! Judges a run purely from its history and the sink's records. An `info` result
//! means the outcome is unknown, so an `info` append may or may not appear in the
//! final read; that one rule keeps the checker honest when a reply is lost after
//! the write landed.
//!
//! - I1 Durability: every `ok` append appears in the final read of its ledger.
//! - I2 No phantom writes: every entry in a final read was invoked on that
//!   ledger, with an `ok` or `info` result (or none at all).
//! - I3 No duplicates: each op_id appears at most once in a final read.
//! - I4 Real-time visibility: a read started after an append returned `ok`
//!   includes that append.
//! - I5 Recovery: every actor a fault interrupted answers again within the
//!   deadline after the fault heals, and every ledger is read back.
//! - I6 Single activation: no two nodes run one session at the same time.
//! - I7 Workflow effects: every session finishes its turns, and a tool call runs
//!   twice only if a fault overlapped its step.
//! - I8 Alarms: each booked alarm fires once and never early, and a booking step
//!   runs twice only if a fault overlapped it.
//!
//! I4 runs when the history has reads, I5 when it has faults, and I6 to I8 when
//! it has agent sessions.

mod ledger;
mod recovery;
mod sessions;

use std::{collections::HashMap, time::Duration};

use chaos_core::{Body, Event, Fault, Network, Op, Proc, SinkRecords, Turn};

pub use recovery::Recovery;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pass,
    Fail,
    /// Nothing contradicted the invariant, but part of it couldn't be checked.
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InvariantResult {
    pub id: &'static str,
    pub name: &'static str,
    pub status: Status,
    /// How many items were checked; each invariant says what it counts.
    pub checked: usize,
    pub violations: Vec<String>,
    /// Things worth knowing that aren't violations, such as reruns a fault excused.
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CheckReport {
    pub invariants: Vec<InvariantResult>,
    /// Ledgers with no successful read issued after their last append.
    pub unreadable: Vec<String>,
    /// Per fault, how long each actor it interrupted took to answer again.
    pub recoveries: Vec<Recovery>,
}

impl CheckReport {
    pub fn verdict(&self) -> Status {
        let statuses = self.invariants.iter().map(|result| result.status);
        if statuses.clone().any(|status| status == Status::Fail) {
            Status::Fail
        } else if statuses.clone().any(|status| status == Status::Unknown) {
            Status::Unknown
        } else {
            Status::Pass
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    /// How long after a fault heals an actor may take to answer again (I5).
    pub recovery_deadline: Duration,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            recovery_deadline: Duration::from_secs(300),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome<'a> {
    /// The history ended before the call did.
    Pending,
    Ok,
    Fail(&'a str),
    Info,
}

/// One client call: its invoke and the result that followed it.
struct Call<'a> {
    op: &'a Op,
    invoke_seq: u64,
    invoke_ns: u64,
    done_ns: Option<u64>,
    outcome: Outcome<'a>,
    op_ids: Option<&'a [String]>,
    turns: Option<&'a [Turn]>,
}

/// A fault's window, numbered in the order the faults started.
#[derive(Debug, Clone, PartialEq)]
struct FaultWindow {
    fault: Fault,
    target: String,
    network: Option<Network>,
    start_ns: u64,
    heal_ns: Option<u64>,
}

impl FaultWindow {
    fn label(&self) -> String {
        match self.network {
            Some(network) => format!("{} {} from {network}", self.fault, self.target),
            None => format!("{} {}", self.fault, self.target),
        }
    }
}

/// Pairs each result with its process's open invoke: one call at a time per
/// process, so a result always belongs to its process's last invoke.
fn calls(events: &[Event]) -> Vec<Call<'_>> {
    let mut open: HashMap<Proc, usize> = HashMap::new();
    let mut calls = Vec::new();
    for event in events {
        let (outcome, op_ids, turns) = match &event.body {
            Body::Invoke { op } => {
                open.insert(event.proc, calls.len());
                calls.push(Call {
                    op,
                    invoke_seq: event.seq,
                    invoke_ns: event.mono_ns,
                    done_ns: None,
                    outcome: Outcome::Pending,
                    op_ids: None,
                    turns: None,
                });
                continue;
            }
            Body::Ok { op_ids, turns, .. } => (Outcome::Ok, op_ids.as_deref(), turns.as_deref()),
            Body::Fail { error, .. } => (Outcome::Fail(error), None, None),
            Body::Info { .. } => (Outcome::Info, None, None),
            Body::FaultStart { .. } | Body::FaultHeal { .. } => continue,
        };
        if let Some(index) = open.remove(&event.proc) {
            let call = &mut calls[index];
            call.done_ns = Some(event.mono_ns);
            call.outcome = outcome;
            call.op_ids = op_ids;
            call.turns = turns;
        }
    }
    calls
}

fn fault_windows(events: &[Event]) -> Vec<FaultWindow> {
    let mut windows: Vec<FaultWindow> = Vec::new();
    for event in events {
        match &event.body {
            Body::FaultStart { fault, target, network } => windows.push(FaultWindow {
                fault: *fault,
                target: target.clone(),
                network: *network,
                start_ns: event.mono_ns,
                heal_ns: None,
            }),
            Body::FaultHeal { fault, target, .. } => {
                if let Some(window) = windows
                    .iter_mut()
                    .find(|window| window.heal_ns.is_none() && window.fault == *fault && window.target == *target)
                {
                    window.heal_ns = Some(event.mono_ns);
                }
            }
            _ => {}
        }
    }
    windows
}

fn status(violations: &[String], unchecked: bool) -> Status {
    if !violations.is_empty() {
        Status::Fail
    } else if unchecked {
        Status::Unknown
    } else {
        Status::Pass
    }
}

/// Seconds since the history started, for messages.
fn secs(ns: impl Into<i128>) -> String {
    format!("{:.1} s", ns.into() as f64 / 1e9)
}

pub fn check(events: &[Event], sink: Option<&SinkRecords>, settings: &Settings) -> CheckReport {
    let calls = calls(events);
    let faults = fault_windows(events);

    let ledgers = ledger::check(&calls);
    let mut invariants = ledgers.invariants;
    let mut recoveries = Vec::new();
    if !faults.is_empty() {
        let (result, measured) = recovery::check(&calls, &faults, &ledgers.unreadable, settings);
        invariants.push(result);
        recoveries = measured;
    }
    if calls.iter().any(|call| matches!(call.op, Op::Send { .. })) {
        invariants.extend(sessions::check(&calls, &faults, sink));
    }

    CheckReport {
        invariants,
        unreadable: ledgers.unreadable,
        recoveries,
    }
}
