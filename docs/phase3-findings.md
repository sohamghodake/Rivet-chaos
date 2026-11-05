# Phase 3 findings

Written 4 November 2026, against Rivet 3.0.0 (source commit 9e01a0a).

## What phase 3 added

**Faults.** All eight core faults run from scenario files in `scenarios/`:

| Fault | Scenario | How it's injected |
| --- | --- | --- |
| F1 | `instance-crash` | `SIGKILL` one instance, start it again after 30 s |
| F1, held | `instance-down-long` | The same, but it stays down for 180 s |
| F2 | `instance-pause` | `docker pause` one instance for 45 s |
| F3 | `instance-replace` | `SIGKILL` one instance, delete its container and log volume, and 30 s later let Compose create a fresh one |
| F4 | `node-crash` | `SIGKILL` one node, start it again after 30 s |
| F5 | `node-isolation` | Disconnect one node from `front` for 60 s |
| F6 | `instance-isolation` | Disconnect one instance from `peer` for 60 s |
| F7 | `bucket-outage` | Disconnect the bucket from `storage` for 120 s |
| F8 | `kill-in-step` | Kill a node the moment one of its workflow steps' tool calls reaches the sink |

A disconnect saves the container's addresses and DNS names on that network and restores them on reconnect, so an instance gets its static peer IP back.

**Invariants.** The checker covers I1 to I8. Each has hand-written good and bad histories in `crates/chaos-checker/tests/`.

**Load.** Besides 50 ledgers, every scenario drives 10 agent sessions. A tenth of ledger calls are reads, for I4. Before the clock starts, every actor is called until it answers once, so runs no longer open with the cluster's first-request rejections (phase 2 had 3 to 10 per run).

**Results.** `rivet-chaos summary <dir>` checks every archived run again and prints one table per scenario and per run; `make phase3` writes it to `results/phase3/summary.md`.

## Decisions worth knowing

**One clock for the whole host.** The wall clock on this host steps backwards every ~29 s (see [phase 2](phase2-findings.md#clock-steps-break-slatedbs-assumptions-upstream-candidate-4)), so nothing compares wall times across processes any more:

- The sink stamps every row with `CLOCK_MONOTONIC`.
- The nodes stamp heartbeats and alarm due times with `process.hrtime`, which reads the same clock.
- At the start of each run the harness measures the offset between the sink's clock and its own over ten round trips, and the checker uses that offset to place sink rows among the history's events.

**F8 lands inside the step every time.** The nemesis arms the sink with `GET /trigger`. The sink reports the next tool call's key and node at once, and holds that call's reply for 3 s. The nemesis kills the node within milliseconds, so the node dies after the side effect and before the step can save its result, which is exactly the case I7 is about.

**Heartbeats survive isolation.** A node cut off from `front` can't reach the sink, so heartbeats queue on the node (up to 20 minutes' worth) and are delivered when it reconnects, each with the time it was taken. Otherwise a split brain during F5 would be invisible to I6.

**I8 and the open question from phase 1.** Phase 1 proposed giving I8 the same exception as I7, because a crash can rerun the step that books an alarm. Each alarm now carries the due time the booking step computed, which separates the two cases:

- *One booking that fires twice* has the same due time on both firings. That is always a violation.
- *A booking step that ran twice* books two alarms with different due times. That is allowed only when a fault overlapped the step, the same rule as I7, and the report says how many there were.

So I8 stays strict about delivery, as the plan says, and step reruns are judged in one place. Change it in `crates/chaos-checker/src/sessions.rs` if you'd rather count every rebooking as a failure.

**Admin tokens in archived runs.** Request URLs carry the cluster's admin token (`rvt-token=`), so it appears in HAProxy's access logs and in error messages. `cluster/up.sh` now makes a new token for every cluster, so a token in an archived run opens nothing, and the driver strips it from the errors it records. Runs archived before this change share one older token, which no cluster accepts any more.

## A fresh cluster can keep every tablet on one instance

The strongest finding so far, and new. Details, evidence and the repro are in [repro/early-placement](../repro/early-placement/README.md); the issue draft is [upstream/early-placement.md](upstream/early-placement.md).

- About 60 s after the engines start, the coordinator places the cluster's 32 tablets on the workers it has admitted by then. In 4 of 19 fresh clusters that was one worker, in 6 it was two, although all three engines started within a second. Rivet's VM guide says starting within a minute makes the first placement use every instance.
- With one worker admitted, every tablet's only copy is on that instance until the others are admitted about 10 s later and learners copy the tablets, 10 to 70 s after that.
- Killing that instance in the window took every actor down until it returned, and afterwards placement stayed at 46 of 96 replicas for at least five minutes.
- Destroying it with its disk and starting a fresh one left the whole cluster unavailable: no actor answered for the 7.5 minutes we watched, and the survivors logged `no live member can take the grant; the tablet stays unavailable until a member returns` for all 32 tablets. Whether the 100 writes acknowledged in the window survive in the bucket couldn't be checked.
- The cause is deliberate: the coordinator bootstraps on whatever workers qualify, so a single-worker development setup gets tablets (`placement.rs:635`). A worker qualifies 60 s after the coordinator first sees it, and workers started together were first seen about 10 s apart.

The harness now avoids the window: `cluster/up.sh` runs `cluster/settle.sh`, which waits until the coordinator reports every replica placed on all three instances. Phase 2's runs didn't wait; in its one single-instance cluster the copies finished about 40 s before the first kill.

## #5957 on a settled cluster

`repro/5957` kills the engine that holds a node's envoy WebSocket. Its first two attempts showed different things:

- The first killed engine-1 about 70 s after the cluster started, before storage settled, and hit the early-placement problem above instead: every ledger unavailable for 210 s, with no stop-command resends at all.
- The second waited for storage to settle. Both nodes' envoys were on engine-1; after the kill, the survivors resent the stop command to the dead engine 20 times, but every ledger answered again within 40 s, as `pegboard.node_lost_threshold` predicts.

So on a settled cluster, #5957 is intermittent, as the issue says (1 of 4 runs), and holding the envoy connection isn't enough on its own. The script now tries up to four fresh clusters, and the five `instance-down-long` runs in the batch measure how often it happens.

## Upstream

Drafts are in [upstream/](upstream/README.md): the early-placement issue, the `http://` S3 endpoint issue (confirmed on main), the `rivetkit-client` README PR, and the SlateDB clock issue, which still needs a repro on an ordinary Linux host. The self-hosting docs' config keys were fixed upstream on 3 November.

## Triage so far

Two of the first five `instance-crash` runs failed I8. Both are understood; one points at something real.

**Seed 1: an alarm 53 ms early by the monotonic clock.** RivetKit books `schedule.after` at the wall clock's now plus the delay, and fires when `trigger_at <= now` by the wall clock (`internal_storage/queries.rs`, `TAKE_DUE_SCHEDULES_SQL`), with no tolerance. Firing early by the monotonic clock therefore means the wall clock moved forward in between, which this host's clock adjustments do. I8 now judges "never early" by the wall clock, the scheduler's own contract, and notes monotonic-early alarms instead. Re-checked, seed 1 passes.

**Seed 5: an actor's storage stalled for about three minutes.** Session 8's turn 57 had its alarm booked three times, at 230, 290 and 350 s, with no fault near those times; the first two bookings fired only at 350 s, 119 and 59 s late. node-a's log shows why: three SQLite transactions on that actor each took exactly 60,002 ms and failed (`slow sqlite transaction`, then `sqlite operation failed` on the scheduler's table), so the workflow step retried and booked again each time. The first hung transaction started at about 170 s, 10 s before the second kill at 180 s, and the stall lasted until about 350 s, more than two minutes after that fault healed at 217 s. Around 150 to 160 s, after the first fault had healed, engine-1 logged `served peer channel replies reached their byte cap` and engine-3 `actor did not become ready before the gateway deadline`. To triage once the batch is done: rerun seed 5, find the tablet holding that actor's volume, and line its log up with the stall.

## The batch

All 45 runs finished on 5 November. Every report was regenerated with the I8 change above, and `results/phase3/summary.md` has the full table. In short:

| Fault | Passed | What failed |
| --- | ---: | --- |
| F1 instance-crash | 4 of 5 | I8 once: the storage stall above |
| F1, held | 5 of 5 | #5957 didn't happen once storage had settled; the slowest recovery was 9.8 s |
| F2 instance-pause | 0 of 5 | Engines fenced and exited; see below. Rerun with supervised engines |
| F3 instance-replace | 5 of 5 | |
| F4 node-crash | 5 of 5 | |
| F5 node-isolation | 0 of 5 | I6 in all five, a workload artifact; I8 once, the lost-alarm pattern |
| F6 instance-isolation | 1 of 5 | I8 three times, lost alarms; I5 once, a session stuck for 302 s after the heal; I7 once |
| F7 bucket-outage | 0 of 5 | Every engine fenced and exited; see below. Rerun with supervised engines |
| F8 kill-in-step | 5 of 5 | |

### F2 and F7: engines fence themselves, and nothing restarted them

During the bucket outage, each engine's topology lease renewal waits on the bucket. After about a minute, every engine logged `topology fenced this worker: lease expired before a heartbeat renewal confirmed. Renewals waited on the object store` and exited (`exiting process so its heartbeat lifetime ends too`). All three did, in all five runs. No append succeeded after the bucket came back: 0 of 13,565 in seed 1.

Pausing one engine for 45 s made a second, unpaused engine fence and exit as well, in every run: `lease expired before a heartbeat renewal confirmed`, then `revoked singleton scope did not join within the stop timeout; exiting process`. With two of three engines gone, nothing answered for the rest of the run. In seed 2 the nemesis then failed to pause engine-3, because it had already exited.

Two separate things are going on:

- **Exiting is Rivet's design.** The VM guide runs the engine under systemd with `Restart=always`, so a fenced engine comes straight back. Our Compose file had `restart: "no"`, so that the nemesis alone decides when a container runs. Engines now have `restart: always`; `docker kill` from the nemesis still counts as a manual stop, so F1 and F4 are unaffected (checked: a killed engine stayed down, an engine that exited by itself was back within 10 s). F2 and F7 are being rerun. The unsupervised runs are kept in `results/phase3-unsupervised/`.
- **What triggers it is a finding either way.** The storage docs say the bucket isn't on the write path, yet a two-minute bucket outage fences every instance, because the topology lease is renewed through the bucket. And pausing one instance for 45 s fences a second one.

### F5: the "split brain" is the workload's heartbeat

Both nodes heartbeated for the same session for up to 223 s, but the tool calls show a clean handoff: each session's work ran on one node at a time. When a node loses the control plane, RivetKit stops its actors through a path that skips `onSleep` (`task.rs`: a closed dispatch inbox returns `LiveExit::Terminated`, which fires the actor's abort signal but not the shutdown hooks), and the session's heartbeat timer was cleared only in `onSleep`. It now also stops on `c.abortSignal`. F5 needs a rerun. RivetKit's docs say `onSleep` is "called when the actor is stopping or sleeping", which this path doesn't do.

### F5 and F6: alarms booked as a partition starts never fire

I8 reported 19 missing alarms in four runs. 17 belong to turns whose tools ran within a second of a partition starting (at 60 s or 210 s); the other 2 are from F6 seed 3's session 5, booked just after the second heal, the session that then stayed stuck for 302 s. Later alarms of the same sessions fired normally after the heal. At those moments the nodes logged SQLite commits expiring after 30 s with `Indeterminate("commit")`, and some sessions' workflows failed with `SQLite transaction coordinator is closed`. The scheduler dispatches every overdue event whenever a later alarm fires, so the lost bookings can't still be in the actors' schedule tables.

Either a booking was lost while the workflow recorded its step as done, which would be a durability bug, or the workflow dropped the failed step. Next: a shrunk repro, with an actor that books alarms in plain actions and a peer partition timed to the bookings, to see whether an acknowledged booking can be lost.

### Lost alarms, confirmed

`repro/lost-alarms` takes the workflow out: 20 actors book alarms from a plain action while one engine is cut off from its peers for 60 s, twice. Acknowledged bookings that never ran: 29 of 13,671 on 3.0.0, and 13 of 14,089 with engine 3.0.5. Every lost booking came due in the first moments of a partition. RivetKit claims a due one-shot by deleting it and commits that before running the action; at the partition those commits come back indeterminate after 30 s and the actor's database closes, which fits a claim that landed without its action ever running. rivet-dev/rivet#5833 covers the same design for an ordinary error and sets aside this crash window; there, a maintainer asked for a real use case rather than an injected failure, which this is. Issue draft: [upstream/lost-alarms.md](upstream/lost-alarms.md).

### Early placement, retested on 3.0.5

A maintainer fixed an availability-zone bug in 3.0.4 that could allow it and asked for a retest. On 3.0.5 the first fresh cluster bootstrapped on one instance, and destroying it left every actor unavailable, exactly as on 3.0.0. The fix only applies when zones are configured. See [discord-answers.md](discord-answers.md) and [upstream/early-placement.md](upstream/early-placement.md#retest-on-305).

## Where this stopped (5 November, 04:30 UTC)

The machine had to shut down. To resume:

1. **F2 and F7 with supervised engines.** Seed 1 of F2 is done and passed every invariant: the cascade fence still happened, but the restarted engine brought the cluster back within 4.2 s of the heal. The rest is left: `make phase3 PHASE3_SCENARIOS="instance-pause bucket-outage" PHASE3_SEEDS="2 3 4 5"` for F2, then all five seeds of F7. A half-finished seed 2 is in `results/aborted/`.
2. ~~**Lost alarms.**~~ Done on 5 November: reproduced on 3.0.0 and 3.0.5 (see above).
3. **F5 again**, now that the heartbeat stops on the actor's abort signal.
4. **The rest of the triage:** the F1 seed 5 storage stall, the F6 session stuck for 302 s, and whether I7 should excuse a rerun right after a heal.
5. **One clean batch** of everything with the final setup, ideally on a Linux VM, then `make dashboard`, the final table, and the findings.
