//! The checker against hand-written histories, one per way a run can go wrong.

use std::path::Path;

use chaos_checker::{CheckReport, Status, check};

fn check_fixture(name: &str) -> CheckReport {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/histories").join(name);
    let history = chaos_history::read(&path).unwrap();
    check(&history.events)
}

fn statuses(report: &CheckReport) -> Vec<(&'static str, Status)> {
    report.invariants.iter().map(|result| (result.id, result.status)).collect()
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
        [("I1", Status::Pass), ("I2", Status::Pass), ("I3", Status::Pass)]
    );
    assert_eq!(report.verdict(), Status::Pass);
    // One acknowledged append checked, two entries read.
    assert_eq!(report.invariants[0].checked, 1);
    assert_eq!(report.invariants[1].checked, 2);
}

#[test]
fn lost_write_fails_durability_only() {
    let report = check_fixture("lost-write.jsonl");
    assert_eq!(
        statuses(&report),
        [("I1", Status::Fail), ("I2", Status::Pass), ("I3", Status::Pass)]
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
