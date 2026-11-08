# Issue draft: one-shot schedules lost at a partition

For rivet-dev/rivet. Not filed. Before filing: search the issues again, and push this repo so the links work.

---

**Title:** One-shot `schedule.after` actions that come due as an engine is cut off from its peers are lost without running

## Environment

- `rivetdev/engine:3.0.0` (and 3.0.5, see below) and rivetkit 3.0.0. The scheduler code (`rivetkit-core/src/actor/schedule.rs`, `internal_storage/queries.rs`) is unchanged between 3.0.0 and 3.0.5.
- Three engines in Docker Compose behind HAProxy, two RivetKit nodes, an S3-compatible bucket. Storage had fully settled (every tablet on three instances) before any fault.

## What happens

An actor books a one-shot schedule from a plain action:

```ts
book: async (c, id: string, delayMs: number) => {
	await c.schedule.after(delayMs, "fire", id);
	return "booked";
},
```

`fire` posts to an external sink the moment it runs. 20 such actors booked alarms 2 s ahead every ~100 ms while one engine was disconnected from the peer network for 60 s, twice.

29 of 13,671 acknowledged bookings never ran. All 29 were acknowledged between 1.6 s before and 0.2 s after a partition began, so each came due in the first moments of a partition. One actor lost all 12 bookings it made in that window. Every booking acknowledged more than 0.2 s into a partition ran.

With `rivetdev/engine:3.0.5` the same run lost 13 of 14,089 acknowledged bookings, with the same timing.

We first saw it in real actor code: an AI-agent workflow that books a follow-up with `schedule.after` after each turn. 19 follow-ups across 4 of 10 partition runs never fired. 17 were booked within a second of a partition starting; the other 2 came just after a heal, in a session that then stayed stuck for five minutes.

## What the logs show

At the partition, SQLite commits in flight from the actors stall for 30 s, then:

```text
warn  sqlite request expired at its deadline; if was_sent=true this indicates an abandoned in-flight request  kind=commit was_sent=true age_ms=30001
error sqlite atomic commit failed  err="Indeterminate(\"commit\")"
warn  slow sqlite transaction  duration_ms=30004 statement_count=3
error sqlite rollback after failed commit failed  rollback="closed: SQLite database is closed."
```

## Why we think it happens

`take_due_schedule_dispatches` claims due one-shots by deleting them (`claim_one_shots_sql`), commits, and then dispatches. If that commit lands but comes back indeterminate, and the actor's database is closed before dispatch, the event is gone and the action never runs. #5833 describes the same claim-then-dispatch shape for an ordinary error and explicitly sets aside "the crash window inherent to claim-then-dispatch". This is that window, opened by a routine peer partition rather than a crash, and held open for the 30 s commit deadline. We haven't inspected the schedule table to prove the delete landed.

This is a different path from #5975, where a sleeping actor's alarm comes due while the engine is down and is consumed in pegboard with no envoy connected. Here the actors are awake, the engine stays up, and the event is claimed in rivetkit-core.

The schedule docs say schedules "survive sleep, restarts, upgrades, and crashes", and that "a failed one-shot is complete after its attempted invocation". Here, as far as we can see, the action is never invoked.

## Question

Is at-most-once delivery across this window intended? If it is, the docs could say so and advise idempotent re-booking. If not, one option is to mark the claim (or record the attempt) in the same transaction that dispatches, so a claim never outlives an undelivered action.

## Repro

`repro/lost-alarms/repro.py` in https://github.com/sohamghodake/Rivet-chaos (about ten minutes, Docker and Python only), with the logs from both runs in `results/probes/lost-alarms-1793905653/` (3.0.0) and `results/probes/lost-alarms-3.0.5-1793906065/` (3.0.5).
