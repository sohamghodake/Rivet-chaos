# rivet-chaos

[![CI](https://github.com/sohamghodake/Rivet-chaos/actions/workflows/ci.yml/badge.svg)](https://github.com/sohamghodake/Rivet-chaos/actions/workflows/ci.yml)

Runs an AI agent workload on a three-instance [Rivet](https://github.com/rivet-dev/rivet) 3.0 cluster, injects faults, and checks a log of every operation against Rivet's documented guarantees, in the style of Jepsen. The design and schedule are in [the plan](docs/plan.md).

**Status:** phase 3. The harness injects all eight core faults (F1 to F8) and checks all eight invariants (I1 to I8). 45 seeded runs, about 1,000,000 acknowledged writes.

## Findings

Tested against Rivet 3.0.0, and retested on 3.0.5 where noted. Each finding has evidence, logs and a repro in this repo.

| Finding | Seen on | Evidence |
| --- | --- | --- |
| **A fresh cluster can bootstrap every storage tablet on one instance.** The docs say starting all instances within a minute makes the first placement use all of them, but in 4 of 19 fresh clusters (started within a second of each other) all 32 tablets went to one instance. Replacing that instance and its disk left every actor in the cluster unavailable for as long as we watched (7.5 min). Raised with the maintainers on Discord. It still happens on 3.0.5, which has their availability-zone fix; that fix only applies when zones are configured. | 3.0.0, 3.0.5 | [repro](repro/early-placement/README.md), [issue draft](docs/upstream/early-placement.md) |
| **One-shot `schedule.after` actions that come due as an engine is partitioned from its peers are lost without running.** 29 of 13,671 acknowledged bookings never ran on 3.0.0, 13 of 14,089 on 3.0.5, all due in the first moments of a partition. First seen as AI-agent workflow follow-ups that never fired. | 3.0.0, 3.0.5 | [repro](repro/lost-alarms), [issue draft](docs/upstream/lost-alarms.md) |
| **A bucket outage fences every engine.** The storage docs say the bucket isn't on the write path, but the topology lease is renewed through it: a 2-minute outage made all three engines fence themselves and exit, in 5 of 5 runs. Pausing one engine for 45 s fenced a second one. With engines supervised (`Restart=always`, as Rivet's VM guide does), the cluster recovers and no acknowledged write is lost, but in one run some actors' storage kept failing for about 2.5 minutes after the bucket came back, delaying their alarms by up to 120 s. | 3.0.0 | [phase 3 findings](docs/phase3-findings.md#f2-and-f7-engines-fence-themselves-and-nothing-restarted-them) |
| **A node cut off from the engine keeps running its actors after it rejoins.** In all five node-isolation runs, the old instances on the node that was cut off kept running (their timers firing) for 40 to 215 s after the engine had started the same actors on the other node. Their writes are fenced (`actor is not running on this envoy`), so no data is lost, but user code runs on two nodes at once. Being shrunk to a repro. | 3.0.0 | [phase 3 findings](docs/phase3-findings.md#f5-the-cut-off-nodes-actors-keep-running-after-it-rejoins) |
| **[rivet-dev/rivet#5957](https://github.com/rivet-dev/rivet/issues/5957) still happens on 3.0.0 final**: survivors keep resending a stop command to a dead engine and the actor stays down until it returns. Intermittent on a settled cluster, as the issue says. | 3.0.0 | [repro](repro/5957/README.md), [phase 2 findings](docs/phase2-findings.md) |
| Smaller ones: the engine's S3 client refuses `http://` endpoints without `AWS_ALLOW_HTTP=true` (confirmed on main), and SlateDB looks sensitive to the wall clock stepping backwards (needs a repro on an ordinary Linux host). The self-hosting docs' rejected config keys were fixed upstream on 3 November. | 3.0.0 | [upstream drafts](docs/upstream/README.md) |

**What held up.** Once storage had settled, no acknowledged write was lost or duplicated in any run where the cluster could be read back. Pausing an engine (F2, with supervised engines), instance replacement (F3), node crashes (F4) and killing a node inside a workflow step (F8) passed every invariant in 5 of 5 runs each: after a mid-step kill the workflow resumed on the other node and finished.

### Phase 3 results

| Fault | Passed | What failed |
| --- | ---: | --- |
| F1 instance-crash | 4 of 5 | I8 once: an actor's storage stalled for about three minutes, still being triaged |
| F1, held | 5 of 5 | |
| F2 instance-pause | 5 of 5 | |
| F3 instance-replace | 5 of 5 | |
| F4 node-crash | 5 of 5 | |
| F5 node-isolation | 0 of 5 | I6 in all five: the cut-off node keeps running its actors after it rejoins |
| F6 instance-isolation | 2 of 5 | I8 three times, lost alarms; I5 once, a session stuck for 302 s after the heal |
| F7 bucket-outage | 4 of 5 | I8 once: storage kept failing for about 2.5 min after the bucket came back |
| F8 kill-in-step | 5 of 5 | |

F2, F5 and F7 were rerun on 8 November with supervised engines and the workload's heartbeat fix, and every run was checked again after two checker fixes; see [the reruns](docs/phase3-findings.md#reruns-8-november).

Every run is archived under `results/`, with its report, operation history, sink records and container logs. Per-run tables are in [results/phase3/summary.md](results/phase3/summary.md); the unsupervised F2 and F7 runs are kept in `results/phase3-unsupervised/`, and the F5 runs from before the heartbeat fix in `results/phase3-old-heartbeat/`. The write-ups are [phase 1](docs/phase1-findings.md), [phase 2](docs/phase2-findings.md) and [phase 3](docs/phase3-findings.md).

## How it works

| Crate | What it does |
| --- | --- |
| `chaos-driver` | Starts a fresh cluster, drives 50 ledger actors and 10 agent sessions, records every call with its result |
| `chaos-nemesis` | Injects the scenario's faults with Docker: kills, pauses, replacements and per-network disconnects |
| `chaos-history` | The operation log, stamped with one monotonic clock for the whole host |
| `chaos-checker` | Checks the history and the sink's records against I1 to I8 |
| `chaos-report`, `chaos-cli` | Writes each run's report, the summary tables and the dashboard's data |
| `effects-sink`, `mock-llm` | Record every tool call, heartbeat and alarm; answer the agents' LLM calls. Never faulted |

The workload in `workload/` is RivetKit 3.0 TypeScript: ledger actors that take appends and reads, and agent sessions that run multi-turn workflows, call tools through the sink and book follow-up alarms.

| Invariant | What it checks |
| --- | --- |
| I1 Durability | Every acknowledged append appears in the final read of its ledger |
| I2 No phantom writes | Every entry in a final read was invoked |
| I3 No duplicates | Each operation ID appears at most once |
| I4 Real-time visibility | A read started after an append returned includes that append |
| I5 Recovery | Every actor answers again within 5 minutes of the fault healing |
| I6 Single activation | No two nodes heartbeat for the same actor in the same 250 ms window |
| I7 Workflow effects | Every session finishes; a tool call runs twice only if a fault overlapped that step |
| I8 Alarms | Each booked alarm fires once, never early, and after recovery if it came due during a fault |

Each invariant has hand-written good and bad histories in `crates/chaos-checker/tests/`.

## Next

- Shrink the F5 overlap to a repro: one actor with a timer on a node cut off past `pegboard.node_lost_threshold`.
- Finish triaging the F1 storage stall and the F6 session stuck after a heal.
- One clean batch of every scenario on a Linux VM, then publish the dashboard.
- The remaining faults in the plan: disk pressure (F9), a slow, lossy network (F10) and a rolling restart (F11).

## Requirements

Docker with Compose, Rust, pnpm and `openssl`. pnpm downloads the pinned Node 24.

```bash
pnpm install
```

## Run a scenario

Each run starts a fresh cluster, drives ledgers and agent sessions while faults are injected, reads every ledger back, waits for every session to finish, checks the invariants, writes `results/<run>/report.md`, archives the history, the sink's records and every container's logs next to it, and tears the cluster down.

```bash
make run SCENARIO=scenarios/instance-crash.toml SEED=7
```

| Scenario | Fault |
| --- | --- |
| `instance-crash` | F1: SIGKILL an instance, restart it 30 s later |
| `instance-down-long` | F1, held: the instance stays down for 3 minutes |
| `instance-pause` | F2: `docker pause` an instance for 45 s |
| `instance-replace` | F3: delete an instance and its volume, start a fresh one |
| `node-crash` | F4: SIGKILL a node, restart it 30 s later |
| `node-isolation` | F5: cut a node off `front` for 60 s |
| `instance-isolation` | F6: cut an instance off `peer` for 60 s |
| `bucket-outage` | F7: cut the bucket off `storage` for 2 minutes |
| `kill-in-step` | F8: kill a node the moment its workflow step's tool call reaches the sink |

`make phase3` runs every scenario with seeds 1 to 5 into `results/phase3/` and writes one table of them to `results/phase3/summary.md`; `make summary RESULTS=<dir>` prints that table for any folder of runs. `make phase2` runs `instance-crash` with seeds 1 to 10. `make report RUN=results/<run>` checks an archived run again and rewrites its report. `make up` and `make down` start and stop the cluster on their own.

The exit code is 0 if every invariant passed, 2 if one failed, 3 if one couldn't be checked, and 1 if the harness itself failed.

## Dashboard

`dashboard/` charts every archived run: the results table per scenario, and for each run its append latency and throughput over time with the fault windows shaded, an outage bar per interrupted actor, and the invariants. Every chart has a table view.

```bash
make dashboard
pnpm --dir dashboard preview
```

`make dashboard` writes the data with `rivet-chaos export` and builds a static site into `dashboard/dist`, which any static host can serve. `pnpm --dir dashboard dev` serves it with live reload.

## The cluster

| Service | What it is |
| --- | --- |
| `engine-1` to `engine-3` | Rivet 3.0.0 control plane instances, pinned by digest |
| `haproxy` | Round-robin load balancer on `localhost:6420` |
| `s3` | VersityGW, the S3-compatible bucket |
| `node-a`, `node-b` | RivetKit nodes running the workload actors |
| `effects-sink`, `mock-llm` | Services that are never faulted |

The instances share three networks, so each partition fault can cut one kind of traffic: `front` for client and node traffic, `peer` for traffic between instances, and `storage` for the bucket. Each instance's API is also published on `localhost:7421` to `7423`, and its metrics on `7431` to `7433`.

## Phase 1 check, without Docker

```bash
make phase1
```

This runs the engine binary from npm with one node, the sink and the mock LLM, then checks phase 1's exit criteria.

## Tests

```bash
make test
```

This runs the Rust tests and type-checks the workload and the dashboard. CI also runs `cargo fmt --check` and `cargo clippy -- -D warnings`.

## Troubleshooting

If `cargo` hangs while downloading crates under WSL, turn off HTTP multiplexing:

```bash
export CARGO_HTTP_MULTIPLEXING=false
```

## License

[Apache 2.0](LICENSE), the same as Rivet.
