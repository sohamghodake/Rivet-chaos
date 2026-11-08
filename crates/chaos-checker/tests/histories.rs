//! The checker against hand-written histories, one per way a run can go wrong.

use std::{path::PathBuf, time::Duration};

use chaos_checker::{CheckReport, Settings, Status, check};
use chaos_core::{AlarmRow, Body, EffectRow, Event, HeartbeatRow, SinkRecords, Turn};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/histories")
        .join(name)
}

fn events(name: &str) -> Vec<Event> {
    chaos_history::read(&fixture(name)).unwrap().events
}

fn check_fixture(name: &str) -> CheckReport {
    check(&events(name), None, &Settings::default())
}

fn status_of(report: &CheckReport, id: &str) -> Status {
    report
        .invariants
        .iter()
        .find(|result| result.id == id)
        .unwrap()
        .status
}

fn notes(report: &CheckReport, id: &str) -> Vec<String> {
    report
        .invariants
        .iter()
        .find(|result| result.id == id)
        .unwrap()
        .notes
        .clone()
}

fn statuses(report: &CheckReport) -> Vec<(&'static str, Status)> {
    report
        .invariants
        .iter()
        .map(|result| (result.id, result.status))
        .collect()
}

fn violations(report: &CheckReport, id: &str) -> Vec<String> {
    report
        .invariants
        .iter()
        .find(|result| result.id == id)
        .unwrap()
        .violations
        .clone()
}

#[test]
fn good_history_passes() {
    let report = check_fixture("good.jsonl");
    assert_eq!(
        statuses(&report),
        [
            ("I1", Status::Pass),
            ("I2", Status::Pass),
            ("I3", Status::Pass),
            ("I4", Status::Pass),
            ("I5", Status::Pass)
        ]
    );
    assert_eq!(report.verdict(), Status::Pass);
    // One acknowledged append checked, two entries read.
    assert_eq!(report.invariants[0].checked, 1);
    assert_eq!(report.invariants[1].checked, 2);
}

#[test]
fn lost_write_fails_durability_and_visibility() {
    let report = check_fixture("lost-write.jsonl");
    // The final read started after the append was acknowledged, so it also
    // breaks real-time visibility.
    assert_eq!(
        statuses(&report),
        [
            ("I1", Status::Fail),
            ("I2", Status::Pass),
            ("I3", Status::Pass),
            ("I4", Status::Fail)
        ]
    );
    assert_eq!(
        violations(&report, "I1"),
        ["p0-000002 was acknowledged but is missing from ledger/0"]
    );
}

#[test]
fn phantom_entry_fails_i2() {
    let report = check_fixture("phantom.jsonl");
    assert_eq!(report.invariants[1].status, Status::Fail);
    assert_eq!(
        violations(&report, "I2"),
        ["p9-000001 is in ledger/0 but was never appended"]
    );
}

#[test]
fn applied_failure_fails_i2() {
    let report = check_fixture("failed-but-applied.jsonl");
    assert_eq!(
        violations(&report, "I2"),
        ["p0-000001 is in ledger/0 although its call failed: connection refused"]
    );
}

#[test]
fn write_in_the_wrong_ledger_fails_i1_and_i2() {
    let report = check_fixture("wrong-ledger.jsonl");
    assert_eq!(report.invariants[0].status, Status::Fail);
    assert_eq!(
        violations(&report, "I2"),
        ["p0-000001 is in ledger/1 but was appended to ledger/0"]
    );
}

#[test]
fn duplicate_entry_fails_i3() {
    let report = check_fixture("duplicate.jsonl");
    assert_eq!(
        violations(&report, "I3"),
        ["p0-000001 appears 2 times in ledger/0"]
    );
    assert_eq!(report.verdict(), Status::Fail);
}

#[test]
fn unreadable_ledger_is_unknown_not_a_failure() {
    let report = check_fixture("unreadable.jsonl");
    assert_eq!(report.unreadable, ["ledger/0"]);
    assert_eq!(report.invariants[0].status, Status::Unknown);
    assert_eq!(report.verdict(), Status::Unknown);
}

#[test]
fn read_older_than_the_last_append_does_not_count() {
    // Otherwise the append that finished after the read would look lost.
    let report = check_fixture("stale-read.jsonl");
    assert_eq!(report.unreadable, ["ledger/0"]);
    assert!(violations(&report, "I1").is_empty());
    assert_eq!(report.verdict(), Status::Unknown);
}

#[test]
fn read_missing_an_acknowledged_append_fails_i4() {
    let report = check_fixture("invisible-write.jsonl");
    assert_eq!(status_of(&report, "I1"), Status::Pass);
    assert_eq!(
        violations(&report, "I4"),
        ["read of ledger/0 started at 0.0 s is missing p0-000001 (acknowledged at 0.0 s)"]
    );
    // Four reads, of which only the first is wrong: the third ran alongside an
    // append, so it doesn't have to include it.
    assert_eq!(report.invariants[3].checked, 4);
}

#[test]
fn recovery_is_measured_from_the_first_failure() {
    let report = check_fixture("recovery.jsonl");
    assert_eq!(status_of(&report, "I5"), Status::Pass);
    let recovered: Vec<(&str, f64, f64)> = report
        .recoveries
        .iter()
        .map(|recovery| {
            (
                recovery.actor.as_str(),
                recovery.outage_ns as f64 / 1e9,
                recovery.after_heal_ns as f64 / 1e9,
            )
        })
        .collect();
    // ledger/0 answered while the fault was still in place; ledger/1 waited for it to heal.
    assert_eq!(
        recovered,
        [("ledger/0", 8.5, -19.5), ("ledger/1", 30.0, 5.0)]
    );
}

#[test]
fn recovery_past_the_deadline_fails_i5() {
    let settings = Settings {
        recovery_deadline: Duration::from_secs(2),
    };
    let report = check(&events("recovery.jsonl"), None, &settings);
    assert_eq!(
        violations(&report, "I5"),
        ["ledger/1 answered again 5.0 s after kill engine-1 healed, past the 2.0 s deadline"]
    );
}

#[test]
fn actor_that_never_answers_again_fails_i5() {
    // Without ledger/1's later calls, it failed during the fault and then went quiet.
    let events: Vec<Event> = events("recovery.jsonl")
        .into_iter()
        .filter(|event| !matches!(event.proc, chaos_core::Proc::Client(3 | 4 | 6)))
        .collect();
    let report = check(&events, None, &Settings::default());
    assert_eq!(
        violations(&report, "I5"),
        [
            "ledger/1 never answered again after kill engine-1",
            "ledger/1 was never read back after its last append"
        ]
    );
}

fn sink() -> SinkRecords {
    serde_json::from_str(&std::fs::read_to_string(fixture("sessions.sink.json")).unwrap()).unwrap()
}

fn check_sessions(sink: &SinkRecords) -> CheckReport {
    check(&events("sessions.jsonl"), Some(sink), &Settings::default())
}

fn effect(key: &str, mono_ns: u64) -> EffectRow {
    EffectRow {
        key: key.into(),
        node: "node-b".into(),
        mono_ns,
    }
}

/// An alarm whose wall clock agrees with its monotonic one.
fn alarm(key: &str, due_mono_ns: u64, mono_ns: u64) -> AlarmRow {
    AlarmRow {
        key: key.into(),
        node: "node-b".into(),
        due_mono_ns,
        mono_ns,
        due_ms: (due_mono_ns / 1_000_000) as i64,
        received_ms: (mono_ns / 1_000_000) as i64,
    }
}

#[test]
fn good_sessions_pass() {
    let report = check_sessions(&sink());
    let sessions: Vec<(&str, Status, usize)> = report
        .invariants
        .iter()
        .filter(|result| ["I6", "I7", "I8"].contains(&result.id))
        .map(|result| (result.id, result.status, result.checked))
        .collect();
    // One session heartbeating, three tool calls, two finished turns with alarms.
    assert_eq!(
        sessions,
        [
            ("I6", Status::Pass, 1),
            ("I7", Status::Pass, 3),
            ("I8", Status::Pass, 2)
        ]
    );
    assert_eq!(report.verdict(), Status::Pass);
}

#[test]
fn sessions_without_sink_records_are_unknown() {
    let report = check(&events("sessions.jsonl"), None, &Settings::default());
    assert_eq!(status_of(&report, "I7"), Status::Unknown);
    assert_eq!(report.verdict(), Status::Unknown);
}

#[test]
fn two_nodes_at_once_fail_i6() {
    let mut sink = sink();
    sink.heartbeats.extend((0..9).map(|beat| HeartbeatRow {
        key: "0".into(),
        node: "node-b".into(),
        sent_mono_ns: 2_000_000_000 + beat * 250_000_000,
    }));
    assert_eq!(
        violations(&check_sessions(&sink), "I6"),
        ["session/0 ran on node-a and node-b at once for 2.0 s from 2.0 s"]
    );
}

#[test]
fn quick_handoff_is_not_split_brain() {
    let mut sink = sink();
    // node-b takes over 50 ms after node-a's last heartbeat.
    sink.heartbeats.retain(|beat| beat.node == "node-a");
    sink.heartbeats.extend((0..8).map(|beat| HeartbeatRow {
        key: "0".into(),
        node: "node-b".into(),
        sent_mono_ns: 5_050_000_000 + beat * 250_000_000,
    }));
    assert!(violations(&check_sessions(&sink), "I6").is_empty());
}

#[test]
fn tool_call_rerun_without_a_fault_fails_i7() {
    let mut sink = sink();
    sink.effects.push(effect("0/1/tool-0", 1_700_000_000));
    assert_eq!(
        violations(&check_sessions(&sink), "I7"),
        ["tool call 0/1/tool-0 ran 2 times, from 1.5 s, with no fault during its step"]
    );
}

#[test]
fn tool_call_rerun_across_a_fault_is_allowed() {
    let mut sink = sink();
    sink.effects.push(effect("0/2/tool-0", 9_000_000_000));
    let report = check_sessions(&sink);
    assert_eq!(status_of(&report, "I7"), Status::Pass);
    assert_eq!(
        notes(&report, "I7"),
        ["1 tool calls ran more than once, each with a fault during its step."]
    );
}

#[test]
fn finished_turn_without_its_tool_call_fails_i7() {
    let mut sink = sink();
    sink.effects.retain(|effect| effect.key != "0/1/tool-1");
    assert_eq!(
        violations(&check_sessions(&sink), "I7"),
        ["session/0 finished turn 1 without calling tool-1"]
    );
}

#[test]
fn unfinished_and_unsent_turns_fail_i7() {
    let events: Vec<Event> = events("sessions.jsonl")
        .into_iter()
        .map(|mut event| {
            if let Body::Ok {
                turns: Some(turns), ..
            } = &mut event.body
            {
                *turns = vec![
                    Turn {
                        turn: 1,
                        tool_calls: 2,
                    },
                    Turn {
                        turn: 4,
                        tool_calls: 0,
                    },
                ];
            }
            event
        })
        .collect();
    let report = check(&events, Some(&sink()), &Settings::default());
    assert_eq!(
        violations(&report, "I7"),
        [
            "session/0 never finished turn 2",
            "session/0 finished turn 4, which was never sent"
        ]
    );
}

#[test]
fn missing_early_and_repeated_alarms_fail_i8() {
    let mut sink = sink();
    sink.alarms = vec![
        alarm("0/1", 2_700_000_000, 2_600_000_000),
        alarm("0/1", 2_700_000_000, 2_710_000_000),
    ];
    assert_eq!(
        violations(&check_sessions(&sink), "I8"),
        [
            "alarm 0/1 fired 100 ms before its wall-clock due time",
            "alarm 0/1, booked for 2.7 s, fired 2 times",
            "alarm 0/2 never fired"
        ]
    );
}

#[test]
fn rebooking_is_allowed_only_across_a_fault() {
    let mut sink = sink();
    // Turn 2's booking step ran again after node-a was killed at 5 s.
    sink.alarms.push(alarm("0/2", 9_600_000_000, 9_601_000_000));
    let report = check_sessions(&sink);
    assert_eq!(status_of(&report, "I8"), Status::Pass);
    assert_eq!(notes(&report, "I8").len(), 1);

    // Turn 1's booking step ran twice well before the fault.
    sink.alarms.push(alarm("0/1", 2_900_000_000, 2_901_000_000));
    assert_eq!(
        violations(&check_sessions(&sink), "I8"),
        [
            "alarm 0/1 was booked 2 times, due at 2.7 s, 2.9 s, with no fault during its booking step"
        ]
    );
}

#[test]
fn alarm_early_only_by_the_monotonic_clock_is_a_note() {
    // On time by the wall clock the scheduler uses, 50 ms early by the monotonic
    // one: the wall clock was stepped forward in between.
    let mut sink = sink();
    sink.alarms[0] = AlarmRow {
        mono_ns: 2_650_000_000,
        received_ms: 2_701,
        due_ms: 2_700,
        ..alarm("0/1", 2_700_000_000, 2_650_000_000)
    };
    let report = check_sessions(&sink);
    assert_eq!(status_of(&report, "I8"), Status::Pass);
    assert_eq!(notes(&report, "I8").len(), 1);
}

#[test]
fn a_rerun_inside_a_fault_that_started_earlier_is_excused() {
    // node-a was killed from 5 s to 8 s. Turn 2's tool ran twice, at 6.5 s and
    // 7.5 s: the fault had started before the step, but was in place all along.
    let mut sink = sink();
    sink.effects.retain(|row| row.key != "0/2/tool-0");
    sink.effects.push(effect("0/2/tool-0", 6_500_000_000));
    sink.effects.push(effect("0/2/tool-0", 7_500_000_000));
    assert_eq!(status_of(&check_sessions(&sink), "I7"), Status::Pass);

    // The same for a booking step: two bookings, due at 7.2 s and 7.9 s.
    let sink = sink_with_alarm_rebooked(7_200_000_000, 7_900_000_000);
    let report = check_sessions(&sink);
    assert_eq!(status_of(&report, "I8"), Status::Pass);
    assert_eq!(notes(&report, "I8").len(), 1);
}

#[test]
fn a_rerun_after_the_fault_healed_is_not_excused() {
    // Both bookings are well after the heal at 8 s.
    let sink = sink_with_alarm_rebooked(12_000_000_000, 15_000_000_000);
    assert_eq!(
        violations(&check_sessions(&sink), "I8"),
        [
            "alarm 0/2 was booked 2 times, due at 12.0 s, 15.0 s, with no fault during its booking step"
        ]
    );
}

#[test]
fn alarm_early_only_by_the_wall_clock_is_a_note() {
    // 20 ms early by the wall clock, 40 ms late by the monotonic one: the wall
    // clock was stepped back between the booking and the firing.
    let mut sink = sink();
    sink.alarms[0] = AlarmRow {
        received_ms: 2_680,
        due_ms: 2_700,
        ..alarm("0/1", 2_700_000_000, 2_740_000_000)
    };
    let report = check_sessions(&sink);
    assert_eq!(status_of(&report, "I8"), Status::Pass);
    assert_eq!(notes(&report, "I8").len(), 1);
}

/// The fixture's sink with turn 2's alarm booked twice, at these due times.
fn sink_with_alarm_rebooked(first_due_ns: u64, second_due_ns: u64) -> SinkRecords {
    let mut sink = sink();
    sink.alarms.retain(|row| row.key != "0/2");
    sink.alarms
        .push(alarm("0/2", first_due_ns, first_due_ns + 1_000_000));
    sink.alarms
        .push(alarm("0/2", second_due_ns, second_due_ns + 1_000_000));
    sink
}
