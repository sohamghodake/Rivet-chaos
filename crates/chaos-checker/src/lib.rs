//! Judges a run purely from its history. An `info` result means the outcome is
//! unknown, so an `info` append may or may not appear in the final read; that
//! one rule keeps the checker honest when a reply is lost after the write landed.
//!
//! - I1 Durability: every `ok` append appears in the final read of its ledger.
//! - I2 No phantom writes: every entry in a final read was invoked on that
//!   ledger, with an `ok` or `info` result (or none at all).
//! - I3 No duplicates: each op_id appears at most once in a final read.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use chaos_core::{Body, Event, Op, Proc};

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
    /// How many items were checked: appends for I1, read entries for I2 and I3.
    pub checked: usize,
    pub violations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CheckReport {
    pub invariants: Vec<InvariantResult>,
    /// Ledgers with no successful read issued after their last append.
    pub unreadable: Vec<String>,
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
enum Outcome {
    Pending,
    Ok,
    Fail(String),
    Info,
}

struct Append<'a> {
    actor: &'a str,
    outcome: Outcome,
}

/// The last successful read of a ledger and the seq of the call that made it.
struct Read<'a> {
    invoked_seq: u64,
    op_ids: &'a [String],
}

pub fn check(events: &[Event]) -> CheckReport {
    let mut appends: HashMap<&str, Append> = HashMap::new();
    let mut last_append_invoke: HashMap<&str, u64> = HashMap::new();
    let mut final_reads: HashMap<&str, Read> = HashMap::new();
    // One call at a time per process, so a result belongs to its process's last invoke.
    let mut open_invoke: HashMap<Proc, u64> = HashMap::new();

    for event in events {
        match &event.body {
            Body::Invoke { op } => {
                open_invoke.insert(event.proc, event.seq);
                match op {
                    Op::Append { actor, op_id, .. } => {
                        appends.insert(op_id, Append { actor, outcome: Outcome::Pending });
                        last_append_invoke.insert(actor, event.seq);
                    }
                    Op::Read { actor } => {
                        last_append_invoke.entry(actor).or_insert(0);
                    }
                }
            }
            Body::Ok { op, op_ids, .. } => {
                let invoked_seq = open_invoke.remove(&event.proc).unwrap_or(event.seq);
                match op {
                    Op::Append { op_id, .. } => set_outcome(&mut appends, op_id, Outcome::Ok),
                    Op::Read { actor } => {
                        let op_ids = op_ids.as_deref().unwrap_or_default();
                        final_reads.insert(actor, Read { invoked_seq, op_ids });
                    }
                }
            }
            Body::Fail { op, error } => {
                open_invoke.remove(&event.proc);
                if let Op::Append { op_id, .. } = op {
                    set_outcome(&mut appends, op_id, Outcome::Fail(error.clone()));
                }
            }
            Body::Info { op, .. } => {
                open_invoke.remove(&event.proc);
                if let Op::Append { op_id, .. } = op {
                    set_outcome(&mut appends, op_id, Outcome::Info);
                }
            }
            Body::FaultStart { .. } | Body::FaultHeal { .. } => {}
        }
    }

    // A read that started before the ledger's last append can't vouch for it.
    final_reads.retain(|actor, read| {
        last_append_invoke.get(actor).is_none_or(|&last| read.invoked_seq > last)
    });
    let unreadable: BTreeSet<&str> = last_append_invoke
        .keys()
        .filter(|actor| !final_reads.contains_key(*actor))
        .copied()
        .collect();

    let mut ordered: Vec<(&&str, &Read)> = final_reads.iter().collect();
    ordered.sort_by_key(|(actor, _)| **actor);

    CheckReport {
        invariants: vec![
            durability(&appends, &final_reads, &unreadable),
            no_phantoms(&appends, &ordered, &unreadable),
            no_duplicates(&ordered, &unreadable),
        ],
        unreadable: unreadable.into_iter().map(String::from).collect(),
    }
}

fn set_outcome(appends: &mut HashMap<&str, Append>, op_id: &str, outcome: Outcome) {
    if let Some(append) = appends.get_mut(op_id) {
        append.outcome = outcome;
    }
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

fn durability(
    appends: &HashMap<&str, Append>,
    final_reads: &HashMap<&str, Read>,
    unreadable: &BTreeSet<&str>,
) -> InvariantResult {
    let read_sets: HashMap<&str, BTreeSet<&str>> = final_reads
        .iter()
        .map(|(actor, read)| (*actor, read.op_ids.iter().map(String::as_str).collect()))
        .collect();

    let mut checked = 0;
    let mut unchecked = false;
    let mut violations = Vec::new();
    let mut acknowledged: Vec<(&&str, &Append)> = appends
        .iter()
        .filter(|(_, append)| append.outcome == Outcome::Ok)
        .collect();
    acknowledged.sort_by_key(|(op_id, _)| **op_id);
    for (op_id, append) in acknowledged {
        let Some(read) = read_sets.get(append.actor) else {
            unchecked |= unreadable.contains(append.actor);
            continue;
        };
        checked += 1;
        if !read.contains(*op_id) {
            violations.push(format!(
                "{op_id} was acknowledged but is missing from {}",
                append.actor
            ));
        }
    }
    InvariantResult {
        id: "I1",
        name: "Durability",
        status: status(&violations, unchecked),
        checked,
        violations,
    }
}

fn no_phantoms(
    appends: &HashMap<&str, Append>,
    final_reads: &[(&&str, &Read)],
    unreadable: &BTreeSet<&str>,
) -> InvariantResult {
    let mut checked = 0;
    let mut violations = Vec::new();
    for (actor, read) in final_reads {
        for op_id in read.op_ids {
            checked += 1;
            match appends.get(op_id.as_str()) {
                None => violations.push(format!("{op_id} is in {actor} but was never appended")),
                Some(append) if append.actor != **actor => violations.push(format!(
                    "{op_id} is in {actor} but was appended to {}",
                    append.actor
                )),
                Some(Append {
                    outcome: Outcome::Fail(error),
                    ..
                }) => violations.push(format!(
                    "{op_id} is in {actor} although its call failed: {error}"
                )),
                Some(_) => {}
            }
        }
    }
    InvariantResult {
        id: "I2",
        name: "No phantom writes",
        status: status(&violations, !unreadable.is_empty()),
        checked,
        violations,
    }
}

fn no_duplicates(final_reads: &[(&&str, &Read)], unreadable: &BTreeSet<&str>) -> InvariantResult {
    let mut checked = 0;
    let mut violations = Vec::new();
    for (actor, read) in final_reads {
        let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
        for op_id in read.op_ids {
            checked += 1;
            *counts.entry(op_id).or_default() += 1;
        }
        for (op_id, count) in counts.into_iter().filter(|(_, count)| *count > 1) {
            violations.push(format!("{op_id} appears {count} times in {actor}"));
        }
    }
    InvariantResult {
        id: "I3",
        name: "No duplicates",
        status: status(&violations, !unreadable.is_empty()),
        checked,
        violations,
    }
}
