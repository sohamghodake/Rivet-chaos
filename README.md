# rivet-chaos

Runs an AI agent workload on a three-instance Rivet 3.0 cluster, injects faults, and checks a log of every operation against Rivet's documented guarantees. The design and schedule are in [the plan](<rivet-chaos architecture and phased plan.md>).

**Status:** phase 2. A three-instance cluster runs in Docker Compose, and the harness runs the instance-crash fault (F1) end to end, checking invariants I1 to I3. See the findings for [phase 1](docs/phase1-findings.md) and [phase 2](docs/phase2-findings.md).

## Requirements

Docker with Compose, Rust, pnpm and `openssl`. pnpm downloads the pinned Node 24.

```bash
pnpm install
```

## Run a scenario

Each run starts a fresh cluster, drives load while faults are injected, reads every ledger back, checks the invariants, writes `results/<run>/report.md`, archives the history and every container's logs next to it, and tears the cluster down.

```bash
make run SCENARIO=scenarios/instance-crash.toml SEED=7
```

`make phase2` runs `instance-crash` once for each of seeds 1 to 10. `make report RUN=results/<run>` checks an archived history again and rewrites its report. `make up` and `make down` start and stop the cluster on their own.

The exit code is 0 if every invariant passed, 2 if one failed, 3 if one couldn't be checked, and 1 if the harness itself failed.

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
