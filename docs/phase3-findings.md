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
