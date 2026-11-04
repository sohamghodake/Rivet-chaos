# rivet-chaos

Runs an AI agent workload on a three-instance Rivet 3.0 cluster, injects faults, and checks a log of every operation against Rivet's documented guarantees. The design and schedule are in [the plan](<rivet-chaos architecture and phased plan.md>).

**Status:** phase 3. The harness injects all eight core faults (F1 to F8) on a three-instance cluster in Docker Compose and checks all eight invariants (I1 to I8). See the findings for [phase 1](docs/phase1-findings.md) and [phase 2](docs/phase2-findings.md), and the [#5957 reproduction](repro/5957/README.md).

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

## Troubleshooting

If `cargo` hangs while downloading crates under WSL, turn off HTTP multiplexing:

```bash
export CARGO_HTTP_MULTIPLEXING=false
```
