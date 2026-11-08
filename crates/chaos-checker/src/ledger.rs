//! I1 to I4, on the ledgers.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use chaos_core::Op;

use crate::{Call, InvariantResult, Outcome, secs, status};

/// Violations listed for one read before the rest are counted.
const MISSING_LISTED: usize = 3;

pub(crate) struct LedgerChecks {
    pub invariants: Vec<InvariantResult>,
    pub unreadable: Vec<String>,
}

struct Append<'a> {
    actor: &'a str,
    outcome: Outcome<'a>,
}

/// The last successful read of a ledger.
struct Read<'a> {
    invoke_seq: u64,
    op_ids: &'a [String],
}

pub(crate) fn check(calls: &[Call]) -> LedgerChecks {
    let mut appends: HashMap<&str, Append> = HashMap::new();
    let mut last_append_invoke: HashMap<&str, u64> = HashMap::new();
    let mut final_reads: HashMap<&str, (u64, Read)> = HashMap::new();

    for call in calls {
        match call.op {
            Op::Append { actor, op_id, .. } => {
                appends.insert(
                    op_id,
                    Append {
                        actor,
                        outcome: call.outcome,
                    },
                );
                let last = last_append_invoke.entry(actor).or_default();
                *last = (*last).max(call.invoke_seq);
            }
            Op::Read { actor } => {
                last_append_invoke.entry(actor).or_default();
                // The ledger's final read is the successful read that finished last.
                if let (Outcome::Ok, Some(done_ns)) = (call.outcome, call.done_ns)
                    && final_reads
                        .get(actor.as_str())
                        .is_none_or(|(latest, _)| done_ns >= *latest)
                {
                    let read = Read {
                        invoke_seq: call.invoke_seq,
                        op_ids: call.op_ids.unwrap_or_default(),
                    };
                    final_reads.insert(actor, (done_ns, read));
                }
            }
            Op::Send { .. } | Op::Turns { .. } => {}
        }
    }

    // A read that started before the ledger's last append can't vouch for it.
    let final_reads: HashMap<&str, Read> = final_reads
        .into_iter()
        .map(|(actor, (_, read))| (actor, read))
        .filter(|(actor, read)| {
            last_append_invoke
                .get(actor)
                .is_none_or(|&last| read.invoke_seq > last)
        })
        .collect();
    let unreadable: BTreeSet<&str> = last_append_invoke
        .keys()
        .filter(|actor| !final_reads.contains_key(*actor))
        .copied()
        .collect();

    let mut ordered: Vec<(&&str, &Read)> = final_reads.iter().collect();
    ordered.sort_by_key(|(actor, _)| **actor);

    let mut invariants = vec![
        durability(&appends, &final_reads, &unreadable),
        no_phantoms(&appends, &ordered, &unreadable),
        no_duplicates(&ordered, &unreadable),
    ];
    if calls
        .iter()
        .any(|call| matches!(call.op, Op::Read { .. }) && call.outcome == Outcome::Ok)
    {
        invariants.push(visibility(calls));
    }
    LedgerChecks {
        invariants,
        unreadable: unreadable.into_iter().map(String::from).collect(),
    }
}

fn durability(
    appends: &HashMap<&str, Append>,
    final_reads: &HashMap<&str, Read>,
    unreadable: &BTreeSet<&str>,
) -> InvariantResult {
    let read_sets: HashMap<&str, HashSet<&str>> = final_reads
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
        notes: Vec::new(),
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
        notes: Vec::new(),
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
        notes: Vec::new(),
    }
}

/// I4: every successful read includes every append on its ledger that was
/// acknowledged before the read was invoked. Counts reads.
fn visibility(calls: &[Call]) -> InvariantResult {
    // Per ledger, acknowledged appends in the order they were acknowledged.
    let mut acknowledged: HashMap<&str, Vec<(u64, &str)>> = HashMap::new();
    for call in calls {
        if let (Op::Append { actor, op_id, .. }, Outcome::Ok, Some(done_ns)) =
            (call.op, call.outcome, call.done_ns)
        {
            acknowledged
                .entry(actor)
                .or_default()
                .push((done_ns, op_id));
        }
    }
    for appends in acknowledged.values_mut() {
        appends.sort_unstable();
    }

    let mut checked = 0;
    let mut violations = Vec::new();
    for call in calls {
        let (Op::Read { actor }, Outcome::Ok) = (call.op, call.outcome) else {
            continue;
        };
        checked += 1;
        let seen: HashSet<&str> = call
            .op_ids
            .unwrap_or_default()
            .iter()
            .map(String::as_str)
            .collect();
        let earlier = acknowledged.get(actor.as_str()).map_or(&[][..], |appends| {
            &appends[..appends.partition_point(|(done_ns, _)| *done_ns < call.invoke_ns)]
        });
        let missing: Vec<&(u64, &str)> = earlier
            .iter()
            .filter(|(_, op_id)| !seen.contains(op_id))
            .collect();
        if missing.is_empty() {
            continue;
        }
        let listed: Vec<String> = missing
            .iter()
            .take(MISSING_LISTED)
            .map(|(done_ns, op_id)| format!("{op_id} (acknowledged at {})", secs(*done_ns)))
            .collect();
        let more = match missing.len().saturating_sub(MISSING_LISTED) {
            0 => String::new(),
            more => format!(" and {more} more"),
        };
        violations.push(format!(
            "read of {actor} started at {} is missing {}{more}",
            secs(call.invoke_ns),
            listed.join(", ")
        ));
    }
    InvariantResult {
        id: "I4",
        name: "Real-time visibility",
        status: status(&violations, false),
        checked,
        violations,
        notes: Vec::new(),
    }
}
