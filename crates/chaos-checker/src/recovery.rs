//! I5: every actor a fault interrupted answers again within the deadline after
//! the fault heals, and every ledger is read back in the end.
//!
//! An actor counts as interrupted by a fault if a call to it failed, timed out or
//! never finished, having started between the fault's start and shortly after its
//! heal. It has recovered once a call started after that first failure succeeds.

use std::collections::BTreeMap;

use crate::{Call, FaultWindow, InvariantResult, Outcome, Settings, secs, status};

/// Failures this soon after a heal still belong to the fault: a restarted
/// instance takes a few seconds to rejoin.
const AFTER_HEAL_NS: u64 = 10_000_000_000;

#[derive(Debug, Clone, PartialEq)]
pub struct Recovery {
    /// The fault's position among the run's faults, in start order.
    pub fault: usize,
    pub actor: String,
    /// From the actor's first failed call to the end of its first successful one.
    pub outage_ns: u64,
    /// When it answered again, relative to the heal: negative if it came back
    /// while the fault was still in place.
    pub after_heal_ns: i64,
}

pub(crate) fn check(
    calls: &[Call],
    faults: &[FaultWindow],
    unreadable: &[String],
    settings: &Settings,
) -> (InvariantResult, Vec<Recovery>) {
    let deadline_ns = settings.recovery_deadline.as_nanos() as i64;
    let mut violations = Vec::new();
    let mut recoveries = Vec::new();

    for (index, fault) in faults.iter().enumerate() {
        let Some(heal_ns) = fault.heal_ns else {
            violations.push(format!("{} started at {} and never healed", fault.label(), secs(fault.start_ns)));
            continue;
        };
        let mut first_failure: BTreeMap<&str, u64> = BTreeMap::new();
        for call in calls {
            let failed = matches!(call.outcome, Outcome::Fail(_) | Outcome::Info | Outcome::Pending);
            if failed && call.invoke_ns >= fault.start_ns && call.invoke_ns <= heal_ns + AFTER_HEAL_NS {
                let first = first_failure.entry(call.op.actor()).or_insert(call.invoke_ns);
                *first = (*first).min(call.invoke_ns);
            }
        }
        for (actor, failed_ns) in first_failure {
            let answered = calls
                .iter()
                .filter(|call| call.op.actor() == actor && call.outcome == Outcome::Ok && call.invoke_ns >= failed_ns)
                .filter_map(|call| call.done_ns)
                .min();
            let Some(answered_ns) = answered else {
                violations.push(format!("{actor} never answered again after {}", fault.label()));
                continue;
            };
            let after_heal_ns = answered_ns as i64 - heal_ns as i64;
            if after_heal_ns > deadline_ns {
                violations.push(format!(
                    "{actor} answered again {} after {} healed, past the {} deadline",
                    secs(after_heal_ns),
                    fault.label(),
                    secs(deadline_ns)
                ));
            }
            recoveries.push(Recovery {
                fault: index,
                actor: actor.into(),
                outage_ns: answered_ns - failed_ns,
                after_heal_ns,
            });
        }
    }
    for actor in unreadable {
        violations.push(format!("{actor} was never read back after its last append"));
    }

    let result = InvariantResult {
        id: "I5",
        name: "Recovery",
        status: status(&violations, false),
        checked: recoveries.len(),
        violations,
        notes: Vec::new(),
    };
    (result, recoveries)
}
