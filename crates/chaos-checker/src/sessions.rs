//! I6 to I8, on the agent sessions, from the sink's records. Session `session/3`
//! is key `3` in the sink: heartbeats `3`, alarms `3/<turn>`, tool calls
//! `3/<turn>/tool-<n>`.

use std::collections::{BTreeMap, BTreeSet};

use chaos_core::{AlarmRow, Op, SinkRecords, Turn};

use crate::{Call, FaultWindow, InvariantResult, Outcome, Status, secs, status};

/// Heartbeats from one node further apart than this start a new stretch of it
/// running the session.
const STRETCH_GAP_NS: u64 = 1_000_000_000;
/// Two nodes' stretches may touch for one heartbeat period during a handoff.
const OVERLAP_NS: u64 = 250_000_000;
/// Slack when deciding whether a fault overlapped a step, for the clock offset
/// and the time between a step starting and its tool call landing.
const FAULT_MARGIN_NS: i64 = 1_000_000_000;
/// An alarm is booked about a second (`FOLLOW_UP_MS`) before it is due.
const BOOKING_LEAD_NS: i64 = 2_000_000_000;
/// The scheduler keeps millisecond wall-clock timestamps, so allow for rounding.
const EARLY_TOLERANCE_MS: i64 = 5;
/// An alarm this much ahead of its monotonic due time is worth a note: the
/// scheduler fires by the wall clock, so a forward wall-clock step does it.
const MONOTONIC_EARLY_NS: u64 = 5_000_000;

#[derive(Default)]
struct Session<'a> {
    /// Turns whose send was acknowledged.
    acknowledged: BTreeSet<u32>,
    /// Turns whose send may have arrived: acknowledged, unknown or unfinished.
    sent: BTreeSet<u32>,
    last_send_seq: u64,
    /// The last successful `turns` read: when it was invoked and finished, and what it returned.
    finished: Option<(u64, u64, &'a [Turn])>,
}

impl<'a> Session<'a> {
    /// The finished turns, if the last read of them started after the last send.
    fn final_turns(&self) -> Option<&'a [Turn]> {
        self.finished
            .filter(|(invoke_seq, _, _)| *invoke_seq > self.last_send_seq)
            .map(|(_, _, turns)| turns)
    }
}

fn sessions<'a>(calls: &[Call<'a>]) -> BTreeMap<&'a str, Session<'a>> {
    let mut sessions: BTreeMap<&str, Session> = BTreeMap::new();
    for call in calls {
        match call.op {
            Op::Send { actor, turn } => {
                let session = sessions.entry(actor).or_default();
                session.last_send_seq = session.last_send_seq.max(call.invoke_seq);
                match call.outcome {
                    Outcome::Ok => {
                        session.acknowledged.insert(*turn);
                        session.sent.insert(*turn);
                    }
                    Outcome::Info | Outcome::Pending => {
                        session.sent.insert(*turn);
                    }
                    Outcome::Fail(_) => {}
                }
            }
            Op::Turns { actor } => {
                if let (Outcome::Ok, Some(done_ns)) = (call.outcome, call.done_ns) {
                    let session = sessions.entry(actor).or_default();
                    if session
                        .finished
                        .is_none_or(|(_, latest, _)| done_ns >= latest)
                    {
                        session.finished =
                            Some((call.invoke_seq, done_ns, call.turns.unwrap_or_default()));
                    }
                }
            }
            Op::Append { .. } | Op::Read { .. } => {}
        }
    }
    sessions
}

/// `session/3` is `3` in the sink.
fn sink_key(actor: &str) -> &str {
    actor.strip_prefix("session/").unwrap_or(actor)
}

/// The first fault in place at any moment between `from_ns` and `to_ns` on the
/// history's clock. A fault that started earlier and was still in place counts:
/// a step that runs in the middle of a long outage overlaps it. A fault that
/// never healed lasts to the end of the run.
fn fault_between(faults: &[FaultWindow], from_ns: i64, to_ns: i64) -> Option<&FaultWindow> {
    faults.iter().find(|fault| {
        let start = fault.start_ns as i64;
        let heal = fault.heal_ns.map_or(i64::MAX, |heal| heal as i64);
        start <= to_ns && heal >= from_ns
    })
}

fn without_sink(id: &'static str, name: &'static str) -> InvariantResult {
    InvariantResult {
        id,
        name,
        status: Status::Unknown,
        checked: 0,
        violations: Vec::new(),
        notes: vec!["The run has no sink records to check.".into()],
    }
}

pub(crate) fn check(
    calls: &[Call],
    faults: &[FaultWindow],
    sink: Option<&SinkRecords>,
) -> Vec<InvariantResult> {
    let Some(sink) = sink else {
        return vec![
            without_sink("I6", "Single activation"),
            without_sink("I7", "Workflow effects"),
            without_sink("I8", "Alarms"),
        ];
    };
    let sessions = sessions(calls);
    vec![
        single_activation(sink),
        workflow_effects(&sessions, faults, sink),
        alarms(&sessions, faults, sink),
    ]
}

/// I6: no two nodes send heartbeats for one session at the same time. Counts
/// sessions that sent heartbeats.
fn single_activation(sink: &SinkRecords) -> InvariantResult {
    // Per session and node, the times its heartbeats were taken.
    let mut beats: BTreeMap<&str, BTreeMap<&str, Vec<u64>>> = BTreeMap::new();
    for row in &sink.heartbeats {
        beats
            .entry(&row.key)
            .or_default()
            .entry(&row.node)
            .or_default()
            .push(row.sent_mono_ns);
    }

    let mut violations = Vec::new();
    for (key, nodes) in &beats {
        // Per node, the stretches it was running the session.
        let mut stretches: Vec<(&str, u64, u64)> = Vec::new();
        for (node, times) in nodes {
            let mut times = times.clone();
            times.sort_unstable();
            let mut start = times[0];
            for pair in times.windows(2) {
                if pair[1] - pair[0] > STRETCH_GAP_NS {
                    stretches.push((node, start, pair[0]));
                    start = pair[1];
                }
            }
            stretches.push((node, start, *times.last().unwrap()));
        }
        for (index, (node, start, end)) in stretches.iter().enumerate() {
            for (other, other_start, other_end) in &stretches[index + 1..] {
                let overlap = end.min(other_end).saturating_sub(*start.max(other_start));
                if node != other && overlap > OVERLAP_NS {
                    violations.push(format!(
                        "session/{key} ran on {node} and {other} at once for {} from {}",
                        secs(overlap),
                        secs(sink.history_ns(*start.max(other_start)))
                    ));
                }
            }
        }
    }
    InvariantResult {
        id: "I6",
        name: "Single activation",
        status: status(&violations, false),
        checked: beats.len(),
        violations,
        notes: Vec::new(),
    }
}

/// I7: every acknowledged turn finishes, and each finished turn's tool calls ran
/// once, or more often only when a fault overlapped the step. Counts tool calls.
fn workflow_effects(
    sessions: &BTreeMap<&str, Session>,
    faults: &[FaultWindow],
    sink: &SinkRecords,
) -> InvariantResult {
    let mut effects: BTreeMap<&str, Vec<u64>> = BTreeMap::new();
    for row in &sink.effects {
        effects.entry(&row.key).or_default().push(row.mono_ns);
    }

    let mut checked = 0;
    let mut unchecked = false;
    let mut excused = 0;
    let mut violations = Vec::new();
    let mut notes = Vec::new();
    for (actor, session) in sessions {
        let Some(finished) = session.final_turns() else {
            unchecked = true;
            notes.push(format!(
                "{actor} was never read after its last turn was sent, so its turns weren't checked."
            ));
            continue;
        };
        let done: BTreeSet<u32> = finished.iter().map(|turn| turn.turn).collect();
        for turn in session.acknowledged.difference(&done) {
            violations.push(format!("{actor} never finished turn {turn}"));
        }
        for turn in done.iter().filter(|turn| !session.sent.contains(turn)) {
            violations.push(format!(
                "{actor} finished turn {turn}, which was never sent"
            ));
        }
        for turn in finished {
            for tool in 0..turn.tool_calls {
                checked += 1;
                let key = format!("{}/{}/tool-{tool}", sink_key(actor), turn.turn);
                let mut times = effects.get(key.as_str()).cloned().unwrap_or_default();
                times.sort_unstable();
                match times.as_slice() {
                    [] => violations.push(format!(
                        "{actor} finished turn {} without calling tool-{tool}",
                        turn.turn
                    )),
                    [_] => {}
                    [first, .., last] => {
                        let (first, last) = (sink.history_ns(*first), sink.history_ns(*last));
                        if fault_between(faults, first - FAULT_MARGIN_NS, last + FAULT_MARGIN_NS)
                            .is_some()
                        {
                            excused += 1;
                        } else {
                            violations.push(format!(
                                "tool call {key} ran {} times, from {}, with no fault during its step",
                                times.len(),
                                secs(first)
                            ));
                        }
                    }
                }
            }
        }
    }
    if excused > 0 {
        notes.push(format!(
            "{excused} tool calls ran more than once, each with a fault during its step."
        ));
    }
    InvariantResult {
        id: "I7",
        name: "Workflow effects",
        status: status(&violations, unchecked),
        checked,
        violations,
        notes,
    }
}

/// I8: each finished turn's alarm fires, never early, and each booking fires
/// once. A booking step that ran twice books two alarms; that is allowed only
/// when a fault overlapped it, as for any step. Counts finished turns.
fn alarms(
    sessions: &BTreeMap<&str, Session>,
    faults: &[FaultWindow],
    sink: &SinkRecords,
) -> InvariantResult {
    // Per alarm, its firings grouped by the booking they belong to.
    let mut alarms: BTreeMap<&str, BTreeMap<u64, Vec<&AlarmRow>>> = BTreeMap::new();
    for row in &sink.alarms {
        alarms
            .entry(&row.key)
            .or_default()
            .entry(row.due_mono_ns)
            .or_default()
            .push(row);
    }

    let mut checked = 0;
    let mut unchecked = false;
    let mut rebooked = 0;
    let mut monotonic_early = 0;
    let mut wall_early_only = 0;
    let mut violations = Vec::new();
    for (actor, session) in sessions {
        let Some(finished) = session.final_turns() else {
            unchecked = true;
            continue;
        };
        for turn in finished {
            checked += 1;
            let key = format!("{}/{}", sink_key(actor), turn.turn);
            let Some(bookings) = alarms.get(key.as_str()) else {
                violations.push(format!("alarm {key} never fired"));
                continue;
            };
            for (due, firings) in bookings {
                for fired in firings {
                    // Rows from before the wall-clock fields existed have zeros there.
                    let wall_early =
                        fired.due_ms > 0 && fired.received_ms + EARLY_TOLERANCE_MS < fired.due_ms;
                    let mono_early = fired.mono_ns + MONOTONIC_EARLY_NS < *due;
                    // Early by one clock only is that clock being stepped between the
                    // booking and the firing, not the scheduler firing early.
                    match (wall_early, mono_early) {
                        (true, true) => violations.push(format!(
                            "alarm {key} fired {} ms before its wall-clock due time",
                            fired.due_ms - fired.received_ms
                        )),
                        (true, false) => wall_early_only += 1,
                        (false, true) => monotonic_early += 1,
                        (false, false) => {}
                    }
                }
                if firings.len() > 1 {
                    violations.push(format!(
                        "alarm {key}, booked for {}, fired {} times",
                        secs(sink.history_ns(*due)),
                        firings.len()
                    ));
                }
            }
            if bookings.len() > 1 {
                let first = sink.history_ns(*bookings.keys().next().unwrap());
                let last = sink.history_ns(*bookings.keys().last().unwrap());
                if fault_between(faults, first - BOOKING_LEAD_NS, last + FAULT_MARGIN_NS).is_some()
                {
                    rebooked += 1;
                } else {
                    let dues: Vec<String> = bookings
                        .keys()
                        .map(|due| secs(sink.history_ns(*due)))
                        .collect();
                    violations.push(format!(
                        "alarm {key} was booked {} times, due at {}, with no fault during its booking step",
                        bookings.len(),
                        dues.join(", ")
                    ));
                }
            }
        }
    }
    let mut notes = Vec::new();
    if rebooked > 0 {
        notes.push(format!(
            "{rebooked} alarms were booked more than once because a fault interrupted their booking step; each booking fired once."
        ));
    }
    if monotonic_early > 0 {
        notes.push(format!(
            "{monotonic_early} alarms fired on time by the wall clock but early by the monotonic clock, as a forward wall-clock adjustment causes."
        ));
    }
    if wall_early_only > 0 {
        notes.push(format!(
            "{wall_early_only} alarms fired early by the wall clock but on time by the monotonic clock, as a backward wall-clock step between booking and firing causes."
        ));
    }
    InvariantResult {
        id: "I8",
        name: "Alarms",
        status: status(&violations, unchecked),
        checked,
        violations,
        notes,
    }
}
