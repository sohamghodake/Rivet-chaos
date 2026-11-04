# Phase 2 findings

Written 3 November 2026, against Rivet 3.0.0 (source commit 9e01a0a). Where the source is quoted, main at 4ab7e07 is the same.

## Exit criteria

| Criterion | Result |
| --- | --- |
| 10 instance-crash runs in a row | Pass. `make phase2`, seeds 1 to 10, about 7 minutes a run |
| At least 5,000 appends each | Pass. 26,220 to 31,400 acknowledged per run, 279,238 in all |
| Every run produces a report | Pass. `results/phase2/instance-crash-s<seed>-*/report.md` |
| Invariant failures reproduce or are explained | None to explain. I1, I2 and I3 passed in every run |

| Seed | Instances killed | Acknowledged | `info` | Succeeded during faults | p50 | p99 |
| ---: | --- | ---: | ---: | ---: | ---: | ---: |
| 1 | engine-1, engine-3 | 27,693 | 222 | 67.9% | 7.0 ms | 16.3 ms |
| 2 | engine-3, engine-3 | 31,400 | 132 | 96.7% | 6.9 ms | 16.0 ms |
| 3 | engine-1, engine-2 | 28,763 | 192 | 84.3% | 6.5 ms | 14.4 ms |
| 4 | engine-1, engine-3 | 28,269 | 187 | 87.3% | 7.5 ms | 19.2 ms |
| 5 | engine-3, engine-2 | 26,567 | 227 | 30.0% | 7.7 ms | 64.0 ms |
| 6 | engine-2, engine-2 | 26,220 | 262 | 26.5% | 8.6 ms | 44.2 ms |
| 7 | engine-3, engine-1 | 27,652 | 205 | 71.4% | 7.6 ms | 23.6 ms |
| 8 | engine-1, engine-1 | 27,540 | 236 | 58.4% | 7.5 ms | 16.8 ms |
| 9 | engine-1, engine-1 | 28,139 | 233 | 65.5% | 7.5 ms | 15.1 ms |
| 10 | engine-3, engine-1 | 26,995 | 264 | 42.0% | 7.5 ms | 16.8 ms |

Each run killed an instance with `SIGKILL` at 60 s and 180 s and started it again 30 s later; kill to restart took 32 to 34 s. No call ended as `fail`, because HAProxy always accepted the connection, so every error was recorded as `info`. All 50 ledgers read back in every run, within 1.5 s.

Outside the kill windows and after the first 5 seconds, 225,223 calls had no errors at all.

These ran on 16 vCPUs and 7.4 GiB of RAM under WSL2 with Docker Desktop 29.0.1. Durations and latencies come from the monotonic clock, which on this host runs about 4.4% fast (see [the clock section](#clock-steps-break-slatedbs-assumptions-upstream-candidate-4)), so they overstate real time by that much.

## What killing one instance does

The share of calls that succeeded during faults, from 27% to 97%, mostly measures the driver rather than the cluster. Each of the 16 processes waits the full 5 s timeout when it picks a ledger that is down, so a few dead ledgers stall every process. Counting per ledger is clearer. From 6 s after each kill until the restart:

- 282 of 958 ledger windows (29%) had no successful call at all, close to the one third you'd expect from one of three instances.
- The number varied from 0 to 35 ledgers per kill. Killing engine-3 at 60 s in seed 2 took out nothing.
- Every ledger that was down came back 0.1 to 7.7 s after the instance restarted, never before.

A 30 s kill can't show whether anything would have failed over, because a node's reconnects are refused for about 30 s anyway. So one probe (`results/phase2/probe-long-kill-s5-*`, seed 5, killing engine-3 at 60 s) kept the instance down for 183 s, past `storage.balancing.missing_worker_ms` (120 s):

- I1 to I3 passed again, with 14,206 appends acknowledged.
- 12 of 50 ledgers had no successful call from the kill until 7.6 s after the restart. They didn't come back at 120 s or at any point while engine-3 was down.
- At 120 to 150 s the coordinator changed 121 tablet placements, but no learner phase started until about 35 s after the restart.
- The two survivors logged `actor did not become ready before the gateway deadline` (`last_phase=stopping`) 329 times for exactly 12 actors. They also logged `failed to send stop command; the stop timer resends it` 282 times, every one addressed to the dead engine (`http://10.77.1.13:6421/ups/v1/channel`, `endpoint_unresolved`).
- node-b's envoy WebSocket was on engine-3: HAProxy's log shows its 59 s session ending at the kill, then 30 s of reconnects closed within 0.2 to 5 s.

This is **[rivet-dev/rivet#5957](https://github.com/rivet-dev/rivet/issues/5957)**, reproduced on 3.0.0 final. The issue reports it on 3.0.0-preview.4 on three VMs, in 1 of 4 runs, and guesses it needs the killed engine to hold the node's envoy connection. Our 20 batch kills mostly support that guess, with exceptions:

| Nodes whose envoy was on the killed engine | Kills | Ledgers down until the restart |
| ---: | ---: | --- |
| 0 | 9 | 0, 0, 1, 1, 3, 3, 4, 15, 35 |
| 1 | 11 | 7, 12, 16, 18, 22, 23, 25, 27, 27, 27, 28 |
| 2 | 1 | 0 |

The envoy column comes from HAProxy sessions on the killed engine that closed within 3 s of the kill. The 0-node kills with 15 and 35 down, and the 2-node kill with none, need a closer look before commenting on the issue.

The plan's I5 ("answers again within 5 minutes of the fault healing") passed in every run, but I5 as written can't tell failover from waiting. Phase 3 should report availability per actor rather than the share of calls, and keep the long kill as a scenario.

## Which `info` results were applied after all

`info` means the call may have been applied. The final reads show which ones were:

| Error | Applied | `info` calls |
| --- | ---: | ---: |
| `timed out after 5000 ms` | 511 | 1,617 |
| `guard/service_unavailable` (`depot_endpoint_unresolved`) | 0 | 334 |
| `ups/consumer_closed` ("the operation may have executed") | 7 | 91 |
| `guard/service_unavailable` (`depot_overloaded`) | 0 | 54 |
| `error sending request` (not a refused connection) | 2 | 20 |
| `guard/service_unavailable` (`ups_endpoint_unresolved`) | 0 | 16 |
| `guard/actor_stopped_while_waiting` | 0 | 12 |
| `sqlite/closed` | 0 | 7 |
| `core/internal_error`, `depot/overloaded`, `depot/unavailable`, `502 Bad Gateway` | 0 | 8 |
| `ups/transport` | 1 | 1 |

Timeouts, closed consumers, transport errors and send errors after the connection opened were all sometimes applied, so treating them as `info` is required, not just cautious.

None of the 404 `guard/service_unavailable` rejections was applied. If the code confirms the guard rejects before forwarding anything, they can become `fail`. I2 would then check that they really weren't applied, and processes would stop retiring on them. That's a question for the team, below, not a change yet.

## Clock steps break SlateDB's assumptions (upstream candidate 4)

On this host, `CLOCK_MONOTONIC` gained 58.5 s while the wall clock and an external `Date` header both gained 56 s. Something steps the wall clock back to compensate, about 0.62 s twice every 29 s or so (40 steps in 10 idle minutes). The clocksource is `tsc`. Every container shares the kernel's clock, so every engine sees each step.

Rivet's engine gives SlateDB a clock that returns wall time. `SlateClock::now()` (`engine/packages/env/src/backends/production/slatedb.rs:711`) returns `observe().unix_ms`, which is `SystemTime::now()` (`production/clock.rs`), even though the engine's own `TokioClock` takes care to use `CLOCK_BOOTTIME` and treats any regression as fatal. SlateDB assumes the clock doesn't go backwards:

- Its ticker panics with `elapsed time is negative` (`slatedb-common-0.17.0/src/clock.rs:120`, unchanged in 0.18.0, released 2 November).
- Its write path rejects the tick with `invalid clock tick, must be monotonic`.
- SlateDB's maintainers describe exactly this failure in a review of [slatedb/slatedb#2123](https://github.com/slatedb/slatedb/pull/2123): a clock that moves backwards "can panic a background dispatcher", the database closes, and writes fail with that message.
- SlateDB's own `DefaultSystemClock` avoids it by anchoring wall time once and adding monotonic elapsed time.

Across the 10 runs, the engines logged:

- 54 panics, which killed SlateDB's `compactor`, `compaction_worker` or `l0_manifest_writer` tasks and fenced the tablet 13 times.
- 803 `filesystem apply failed in the store; fencing` (or `flush failed`) events with `invalid clock tick, must be monotonic`, about 80 a run, spread across all three instances.

Each fenced tablet was recovered under a new grant, and clients barely noticed: no errors outside the kill windows, and 29 acknowledgements over 300 ms (worst 0.87 s), 17 of them within 3 s after a fencing. It's a correctness risk rather than a visible failure so far: every step makes the storage layer throw away and rebuild state.

Hosts whose time sync steps the clock (common on VMs, and on any host where NTP corrects a large offset) will hit this. The likely fix is for `SlateClock::now()` to anchor the wall time once and add boot-clock elapsed time, as SlateDB's default clock does, or to never return less than its last value.

Before filing: reproduce it on a normal Linux host by stepping the clock in a disposable VM, since stepping it here would affect every container on the machine. GitHub searches of rivet-dev/rivet for `elapsed time is negative` and `SlateClock` found nothing.

## Other findings

**Startup readiness.** Each run's first 5 s had 3 to 10 `guard/service_unavailable` rejections (`depot_endpoint_unresolved`, `depot_overloaded`), even though `bootstrap.sh` had seen `/health` and created the pool. These are the only errors in the "before faults" window. Phase 3 should warm up by reading every ledger until it answers before starting the clock.

**Memory.** Sampled every 30 s across the batch:

| Service | Peak |
| --- | ---: |
| engine-1 to engine-3 | 320, 393, 469 MiB |
| node-a, node-b | 103, 114 MiB |
| s3 (VersityGW) | 89 MiB |
| haproxy | 85 MiB |
| effects-sink, mock-llm | 2 MiB each |

The host never had less than 4.4 GiB available. Engine memory grew steadily through each run, for example from about 100 MiB to 214 to 306 MiB in seed 1, without levelling off. That's fine at this scale but worth watching in longer phase 3 runs.

**Node crash recovery from phase 1.** The 44 s recovery in phase 1's node-kill probe matches `pegboard.node_lost_threshold`, which defaults to 40 s. The docs' name for it, `envoy_lost_threshold`, is rejected by 3.0.0.

## Changes from the plan

Only ledgers are driven so far; agent sessions join the load with I7 and I8 in phase 3.

| Plan says | Reality | What was done |
| --- | --- | --- |
| MinIO with a bucket-creation job | MinIO no longer publishes images. RustFS let two writers both win an `If-Match` race in one of three rounds, so it can't back a store that relies on conditional writes | VersityGW 1.10.0, pinned by digest, on its POSIX backend. A bucket is a directory, so its entrypoint makes it with `mkdir` |
| (not covered) | The engine's `object_store` client refuses `http://` endpoints without `AWS_ALLOW_HTTP=true` | Set on every engine. See upstream candidate 3 |
| `advertise_address` set to `engine-N:6421` | The peer bind IP defaults to the advertised IP, and DNS advertisements need an explicit bind IP | Static IPs on `peer`, so each peer API listens only there and a peer partition really cuts it |
| Timeouts recorded as `info` | Other errors can be applied too (see above) | Everything except a refused connection is `info` |
| (not covered) | Retrying a request the engine may have received would make the history wrong | HAProxy retries only failed connections (`retry-on conn-failure`) |
| Keep every raw history | A run's history is about 1 MB gzipped, and its logs about 22 MB, mostly HAProxy and VersityGW access lines | Histories and logs are gzipped, about 2.8 MB a run |

## Upstream candidates

Nothing has been filed. Before filing, retest on the latest release and search the issues again.

1. **Self-hosting docs give config the engine refuses** (from phase 1, now wider). **Fixed upstream on 3 November** in rivet-dev/website commit 6417f2d (#173); on 4 November none of these keys appear in the self-host pages any more. Beyond the Docker Compose page, the VM, storage and configuration reference pages use `depot.*`, `auto_provision_tablets`, `tablets_per_core`, `envoy_lost_threshold` and `depot tablet provision`. 3.0.0, 3.0.0-preview.6 and `next` (3.0.1-preview.2) reject them all. The working names are `storage.*`, `auto_provision_partitions`, `node_lost_threshold` and `rivet-engine storage partition provision --partitions --workers`.
2. **`rivetkit-client` README doesn't match the 3.0.0 crate** (phase 1, unchanged).
3. **An `http://` S3 endpoint fails silently.** The engine makes no request, logs no error, and exits with `config operation timed out`. It works with the undocumented `AWS_ALLOW_HTTP=true`. At minimum the docs should say so; better, the engine could pass `allow_http` itself for `http://` endpoints or log why it gave up.
4. **SlateDB gets a wall clock that can go backwards** (above). Needs a repro on a normal Linux host first.
5. **[rivet-dev/rivet#5957](https://github.com/rivet-dev/rivet/issues/5957) still happens on 3.0.0 final** (above). This is a comment on an open issue rather than a new one. The probe's history, logs and the envoy table are the evidence; a standalone repro script would make it stronger.

## Add to the Discord draft

One question: does a `guard/service_unavailable` rejection guarantee the request never reached the actor? It's question 3 in [discord-draft.md](discord-draft.md).

## For phase 3

- Warm every ledger up before the clock starts.
- Report per-actor unavailability for I5, and add the long kill as a scenario.
- Explain the #5957 table's exceptions, then write the step-by-step repro the plan asks for and comment on the issue.
- Move guard rejections to `fail` once question 3 of the [Discord draft](discord-draft.md) is answered.
- Run at least one batch on a Linux host with a steady clock, so the SlateDB fencings don't mix with the injected faults.
