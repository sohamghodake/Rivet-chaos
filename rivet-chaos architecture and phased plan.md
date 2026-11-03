# rivet-chaos: architecture and phased plan

Nov 1, 2026 · @taneesha

## Summary

rivet-chaos runs an AI agent workload on a three-instance Rivet 3.0 cluster on one machine, injects faults, and checks a log of every operation against Rivet's documented guarantees. It is built in five phases: four over 2 to 29 November and one stretch phase in December, with the application going out at the end of phase 3 on 22 November.

**Goals**

1. **Durability.** Prove that every acknowledged SQLite write survives the loss of any one control plane instance. Rivet's storage docs say a write is acknowledged once a majority of the instances holding that volume have it on disk.
2. **Recovery.** Measure how long actors take to answer again after a node, instance or network failure, and how long the cluster takes to rebuild replicas. The docs say about two minutes.
3. **Workflow side effects.** Check that tool calls inside workflow steps don't run again after a crash, except where a crash makes a retry unavoidable.
4. **Upstream value.** Turn anything surprising into a minimal reproduction, an issue, and ideally a PR.
5. **Comparable reporting.** Report results the way Rivet's benchmarking page does: p50 and p99, sample counts, hardware, version, and every failure.

**Non-goals**

- **Throughput benchmarking.** Rivet's own benchmarking page says sustained throughput should be measured separately from latency; this project measures correctness and recovery.
- **Multi-region.** The control plane runs in one region today.
- **Clock skew and power loss.** Containers share the host's clock, and depot's own crash suite also excludes torn writes and power loss.
- **Real LLM calls.** A mock keeps runs repeatable and free.

## System architecture

The system has three zones: the harness that drives and judges each run, the Rivet cluster that faults are injected into, and shared services that are never faulted. Keeping the recorder, the side-effect log and the telemetry store outside the cluster means a fault can never corrupt the evidence used to judge it.

&#91;embedded content: rivet-chaos system architecture · harness, cluster, shared services\]

The driver and nemesis are the only things that touch the cluster; the checker judges a run purely from the history and the sink's records.

The cluster uses three Docker networks, so each partition fault cuts exactly one kind of traffic.

| Network | Who is on it | Traffic | Fault it enables |
| --- | --- | --- | --- |
| `front` | Load balancer, nodes, harness, shared services | Node and client traffic to port 6420 | Isolating one node from the control plane |
| `peer` | The three control plane instances only | Replication and coordination on port 6421, which Rivet says must stay private | Cutting one instance off from the others |
| `storage` | Instances and MinIO | Background copies to the bucket | Bucket outage without stopping MinIO's container |

## Components

Every component below is checked against Rivet's code and docs as of 1 November 2026 (commit 9e01a0a). Rivet 3.0 is still in preview, so pin versions and recheck when you upgrade.

### Cluster (faulted)

| Component | Technology | Settings that matter | Notes |
| --- | --- | --- | --- |
| Control plane, 3 instances | `rivetdev/engine:next`, pinned by digest | `api_peer.advertise_address` set to `engine-N:6421`, the only per-instance difference; the same `auth.admin_token` everywhere; `depot.log_directory` on the instance's own volume; `object_storage.primary.s3` pointing at MinIO | The VM guide says to start all three within about a minute so the first storage placement uses all of them |
| Load balancer | HAProxy | Round-robin to port 6420 on each instance, health check on `/health`, one-hour idle timeout as the VM guide recommends | The single entry point for nodes and the harness |
| Bucket | MinIO, S3-compatible | One bucket, set through the S3 `endpoint` option | Rivet's docs list S3-compatible storage as supported |
| Nodes A and B | Node.js 24 running RivetKit | `RIVET_ENDPOINT` with the admin token, pointing at HAProxy; `RIVET_POOL=default`; OTLP exporter variables | Create the pool with `PUT /pools/default?namespace=default` before nodes start, or the control plane refuses them |

### Workload actors (TypeScript)

**`ledger`**, keyed by ledger ID. It has one SQLite table, `entries(op_id TEXT PRIMARY KEY, value INTEGER, written_at INTEGER)`, and two actions:

- `append(op_id, value)` runs a single autocommit `INSERT` through `c.db.execute`. A repeated `op_id` returns "duplicate" instead of inserting twice.
- `read()` returns every `op_id`.

Unique operation IDs make both lost writes and duplicated writes visible. Rivet's benchmark treats an autocommit insert as durable once a majority of disks have it, which gives the checker a clear contract.

**`agent-session`**, keyed by session ID. Its `run` is a `workflow(...)` from `rivetkit/workflow` that loops over queued user messages:

1. A step calls the mock LLM.
2. One step per tool call posts to the side-effect sink with the key `session/turn/tool`.
3. A step saves the reply to the actor's SQLite database and broadcasts tokens over WebSocket.
4. `c.schedule.after(...)` books a follow-up alarm, which the sink also records when it fires.

The actor also sends a heartbeat with its node ID to the sink every 250 ms. Two nodes heartbeating for the same key at the same time is the split-brain signal.

A Rust port of `ledger` using the Rust SDK (`ctx.sql()`, `ctx.schedule()`) moves into a separate `rust` pool in phase 5.

### Harness (Rust workspace)

| Crate | Job | Key libraries |
| --- | --- | --- |
| `chaos-cli` | The `rivet-chaos run <scenario.toml> --seed N` binary that runs a scenario end to end | `clap` |
| `chaos-core` | Shared types: operation, event, scenario, fault | `serde` |
| `chaos-history` | Append-only JSON Lines history, flushed and fsynced every 100 events | `serde_json` |
| `chaos-driver` | Logical client processes that call actor actions with pacing and timeouts | `tokio`, Rivet's Rust client crate |
| `chaos-nemesis` | Fault injection through the Docker API, and `tc` inside containers for latency | `bollard` |
| `chaos-checker` | The invariants in the next section, unit-tested against hand-written histories | none |
| `chaos-report` | Markdown report and ClickHouse rows | `clickhouse`, `hdrhistogram` |
| `effects-sink` | HTTP service recording tool calls, alarms and heartbeats; can ping the nemesis on the first call of a step | `axum`, `rusqlite` |
| `mock-llm` | Scripted replies with a seeded, configurable delay | `axum` |

The driver should use Rivet's Rust client (`rivetkit-rust/packages/client`). If it doesn't fit 3.0 yet, call actions through the gateway's HTTP interface instead and note it as a finding.

### Observability (never faulted)

- **Traces.** Nodes export spans by setting `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`, which Rivet's tracing docs say enables tracing. An OpenTelemetry Collector forwards them to ClickHouse.
- **Metrics.** The engine has a metrics listener (`metrics.port`). Check its format in phase 2 before planning dashboards around it.
- **Results.** `chaos-report` writes run, operation and fault tables to ClickHouse. A small Vite and TypeScript page charts them, or Grafana if time is short.

## Correctness model

Every client call is recorded as an `invoke` event followed by exactly one result: `ok`, `fail` (definitely not applied), or `info` (outcome unknown, such as a timeout). The checker treats `info` writes as possibly applied. That one rule is what keeps the checker honest when the network drops a reply after the write landed.

```json
{"run":"node-kill-s42","seq":1042,"proc":7,"type":"invoke","f":"append","actor":"ledger/17","op_id":"p7-000213","value":213,"mono_ns":81234567890,"wall_ms":1794043331000}
{"run":"node-kill-s42","seq":1043,"proc":7,"type":"ok","f":"append","actor":"ledger/17","op_id":"p7-000213","mono_ns":81236012345}
{"run":"node-kill-s42","seq":1044,"proc":"nemesis","type":"fault-start","fault":"kill","target":"node-a","mono_ns":81240000000}
```

Three rules keep the history usable:

- **One process, one call at a time.** Each logical client process waits for a result before its next call.
- **Retire a process after `info`.** Its next call gets a new process ID, so the order of operations within a process stays meaningful.
- **Faults go in the same log.** The nemesis writes fault start and heal events, so the checker knows each fault's time window.

### Invariants

| ID | Invariant | Passes when | Uses |
| --- | --- | --- | --- |
| I1 | Durability | Every `ok` append appears in the final read of its ledger | Ledger history, final reads |
| I2 | No phantom writes | Every entry in a final read was invoked, with an `ok` or `info` result | Ledger history, final reads |
| I3 | No duplicates | Each `op_id` appears at most once | Final reads |
| I4 | Real-time visibility | A read started after an append returned `ok` includes that append | Reads during the run |
| I5 | Recovery | Every actor answers again within 5 minutes of the fault healing; the time per actor is reported | Results after each fault |
| I6 | Single activation | No two nodes send heartbeats for the same actor key in the same 250 ms window | Sink heartbeats |
| I7 | Workflow effects | Every session finishes all its turns; a tool call runs twice only if a fault overlapped that step | Sink effect log, fault windows |
| I8 | Alarms | Each scheduled alarm fires once, never early, and fires after recovery if it came due during a fault | Sink alarm log |

I1 to I3 are pass or fail. I5 is a deadline plus a measurement. I7 and I8 depend on Rivet's intended semantics, which are listed under open questions, so treat their first failures as questions for the team before calling them bugs.

### End of a run

After the last fault heals, the driver stops new writes and waits 30 seconds. It then reads every ledger, retrying each until it succeeds or 5 minutes pass. An actor that can't be read by then fails I5 and is reported by key.

## Fault catalogue

Eleven faults, eight of them core. Each is injected through the Docker API, recorded in the history with its time window, and judged against what Rivet's docs say should happen.

| ID | Fault | How it is injected | What Rivet's docs lead you to expect | Invariants | Phase |
| --- | --- | --- | --- | --- | --- |
| F1 | Instance crash | `SIGKILL` one engine container, restart it after 30 s | No acknowledged write is lost when one of three instances goes down | I1–I5 | 2 |
| F2 | Instance pause | `docker pause` for 10 to 60 s, simulating a stalled machine | Depot fences a write handle after 30 s without quorum progress | I1–I6 | 3 |
| F3 | Permanent instance loss | Remove one instance and its log volume, then start a fresh one | Replicas are rebuilt on the remaining instances after about two minutes | I1, rebuild time | 3 |
| F4 | Node crash | `SIGKILL` node A | Its actors restart on node B with state intact | I1, I5–I8 | 3 |
| F5 | Node isolation | Disconnect node A from `front` for 60 s | Actors move to node B, and node A stops serving them | I6 | 3 |
| F6 | Instance isolation | Disconnect one instance from `peer` | The cut-off instance can't acknowledge writes alone | I1–I4 | 3 |
| F7 | Bucket outage | Disconnect MinIO from `storage` for 2 minutes | Writes keep acknowledging, because the bucket isn't on the write path; waking an actor whose data is only in the bucket may fail until it returns | I1, I5 | 3 |
| F8 | Kill inside a workflow step | The sink pings the nemesis on a tool call, which kills that actor's node at once | The workflow resumes on node B and finishes | I7 | 3 |
| F9 | Disk pressure | Put one instance's log directory on a small tmpfs and fill it to 95% | Rivet keeps at least 10% of the disk free, so expect clean errors rather than corruption | I1, I3 | 5 |
| F10 | Slow, lossy network | `tc netem` with 100 ms ± 50 ms delay and 5% loss on `peer` | Higher latency, no correctness change | I1–I4 | 5 |
| F11 | Rolling restart | Restart instances one at a time, as in an upgrade | Short or no unavailability | I1, I5 | 5 |

F8's trigger makes the timing precise. Killing a node at a random moment rarely lands inside a step that lasts milliseconds; killing it the moment the sink sees the tool call lands inside every time.

Faults run on schedules set in each scenario file: once, repeated every N seconds, or combined (for example F4 during F1). Compare F4 and F5 results with the open reports where actors stayed unreachable after a crash ([#5957](https://github.com/rivet-dev/rivet/issues/5957), [#5946](https://github.com/rivet-dev/rivet/issues/5946)).

## Scenario lifecycle

Each run is one scenario file and one seed, so any result can be rerun exactly. Load and faults loop as many times as the schedule asks; the final reads and checks happen once, at the end.

&#91;embedded content: scenario lifecycle · 10 steps, one loop\]

A scenario file sets the workload size, the fault schedule and the check deadlines:

```toml
name = "node-kill"
seed = 42
duration_s = 600

[workload]
ledgers = 50
agent_sessions = 10
processes = 16
pacing_ms = [50, 200]
call_timeout_ms = 5000

[[nemesis]]
fault = "kill"
target = "node-a"
at_s = 120
heal_after_s = 30
repeat_every_s = 120

[checks]
quiesce_s = 30
recovery_deadline_s = 300
```

Teardown runs `docker compose down -v` and archives the history, report, and every container's logs into `results/<run>/`, so a failed run can be attached to an issue as it is.

## Phased plan

Five phases, each ending at a gate with exit criteria you can check. Phase 3's gate is the application; phase 5 is optional and runs after you've applied.

&#91;embedded content: phase roadmap · 5 phases, 4 gates\]

### Phase 1: Foundations and workload (2 to 8 November)

Goal: both actors run on a single local engine, driven by a small Rust client.

- [ ] Rust Book chapters 1 to 10 and the Tokio tutorial
- [x] Create the repo with the layout below: a Cargo workspace and a pnpm workspace
- [x] Run one engine from Rivet's Docker Compose example and create the `default` pool
- [x] Write the `ledger` actor with its SQLite table and `append` and `read` actions
- [x] Write the `agent-session` actor: workflow loop, tool steps, scheduled alarm, broadcasts
- [x] Build `mock-llm` and `effects-sink` with axum
- [x] Spike the Rust client: 1,000 appends from Rust, read back
- [ ] Ask on Discord how they'd run three instances on one host for testing

Exit criteria: 1,000 appends read back exactly; one agent session completes 10 turns; the sink shows one effect per tool step.

### Phase 2: Cluster and harness core (9 to 15 November)

Goal: one command brings up a three-instance cluster, and the harness survives one fault type end to end.

- [x] Compose file with three engines, HAProxy, MinIO with a bucket-creation job, two nodes, and the `front`, `peer` and `storage` networks
- [x] `bootstrap.sh`: wait for `/health` on every instance, then create the pool
- [x] `chaos-history`: JSON Lines writer and reader, with a schema version field
- [x] `chaos-driver`: one Tokio task per logical process, seeded pacing, timeouts recorded as `info`
- [x] `chaos-nemesis` v1: kill and restart a container (F1), logging fault windows
- [x] `chaos-checker` v1: I1 to I3, with unit tests on hand-written good and bad histories
- [x] `chaos-cli` with TOML scenarios and `--seed`
- [x] `chaos-report` v1: Markdown with p50 and p99 append latency before, during and after the fault, the share of calls that succeeded, and invariant results

Exit criteria: 10 instance-crash runs in a row, each with at least 5,000 appends, all produce reports; any invariant failure either reproduces with its seed or is explained.

### Phase 3: Fault catalogue and bug hunt (16 to 22 November)

Goal: all core faults run, and anything surprising becomes a minimal reproduction.

- [ ] Add F2 to F8 to the nemesis, including the sink-triggered kill
- [ ] Add I4 to I8 to the checker
- [ ] Run each fault with 5 seeds and keep every raw history
- [ ] Triage each failure: rerun with the same seed, shrink it (fewer actors, one fault, shorter run), write a standalone repro script, search existing issues, then file
- [ ] Reproduce one existing report (#5957 or #5946) step by step
- [ ] Apply on 22 November with the repo, a report and the issue link

Exit criteria: F1 to F8 all run from scenario files; one results table covers every fault; at least one issue filed, or an existing one reproduced with exact steps.

### Phase 4: Upstream and publish (23 to 29 November)

Goal: turn findings into an upstream contribution and a public write-up.

- [ ] Agree the fix on the issue thread, then open a PR with a regression test, titled `fix(<scope>): …`
- [ ] Export node traces through the OpenTelemetry Collector into ClickHouse
- [ ] Chart page: latency over time with fault windows shaded, and recovery time per fault
- [ ] README: method, hardware, versions, results in Rivet's benchmark format, and known limits
- [ ] One `make run-all` target that reproduces every result

Exit criteria: PR open or merged, README results public, and a stranger can rerun everything with one command.

### Phase 5: Hardening (December, optional)

- [ ] Nightly runs in CI against each new Rivet preview, comparing results across versions
- [ ] F9 to F11
- [ ] Rust `ledger` on its own `rust` pool
- [ ] Cross-actor operations, checked with an Elle-style transaction checker if you add them

## Repository layout

One repo with a Rust workspace for the harness and services, a pnpm workspace for the actors and the chart page, and every result kept under version control.

```text
rivet-chaos/
├── README.md                # method, hardware, versions, results
├── Makefile                 # make up, make run SCENARIO=…, make run-all, make report
├── cluster/
│   ├── compose.yaml         # engine-1..3, haproxy, minio, node-a, node-b, otel, clickhouse
│   ├── engine/              # engine-1.json … engine-3.json, differing only in advertise_address
│   ├── haproxy.cfg
│   └── bootstrap.sh         # wait for /health, create pools
├── workload/                # TypeScript, pnpm
│   └── src/
│       ├── ledger.ts
│       ├── agent-session.ts
│       └── registry.ts
├── crates/                  # Rust workspace
│   ├── chaos-cli/
│   ├── chaos-core/
│   ├── chaos-history/
│   ├── chaos-driver/
│   ├── chaos-nemesis/
│   ├── chaos-checker/
│   ├── chaos-report/
│   ├── effects-sink/
│   └── mock-llm/
├── scenarios/               # instance-crash.toml, node-kill.toml, bucket-outage.toml, …
├── results/                 # one folder per run: history.jsonl, report.md, container logs
└── dashboard/               # chart page, Vite + TypeScript
```

## Risks and mitigations

The biggest risk is the setup, not the code: Rivet's Docker Compose example runs one instance, and its three-instance guides target VMs and Kubernetes.

| Risk | Impact | Mitigation |
| --- | --- | --- |
| Three instances won't run cleanly in Compose | Blocks phase 2 | Ask on Discord in phase 1. Fallback: three engine processes on the host with different ports and log directories |
| Rivet 3.0 preview changes under you | Results stop matching the current release | Pin the image digest and package versions, record the commit in every report, rerun on the latest preview before filing anything |
| Checker bugs report false failures | Wasted time, or a wrong issue filed | Unit-test every invariant on hand-written histories first; triage every failure by rerunning its seed |
| Laptop runs out of memory | Unstable timings, false recovery failures | Measure memory in phase 2; set container limits; move ClickHouse to phase 4; use a Linux cloud VM for the final numbers |
| Docker Desktop networking quirks on macOS or Windows | Partition faults behave differently | Run fault scenarios on Linux |
| The harness grows past four weeks | No application by 22 November | Cut ClickHouse and the chart page first; a clean report on F1, F4 and F7 is enough to apply |
| A finding is already fixed on main | A duplicate issue | Retest on the latest preview and search issues before filing |
| Too many low-value issues | Annoys maintainers | File only shrunk, deduplicated, reproducible findings |

## Open questions

Settle these in phase 1, from the code or by asking on Discord. Several decide whether a checker failure is a bug or expected behaviour.

1. **Crash policy.** Which policy applies to 3.0 actors when their node dies? The `CrashPolicy` type in `engine/packages/types` defaults to `Destroy`, while the README says a failed node's actors are rescheduled with state intact. Issue #5908 reports the policy being ignored on v2.
2. **State durability.** When an action returns, are its `c.state` changes durable, or only SQLite writes? Issue #5942 involves `saveState({ immediate: true })`, which suggests state saves can be deferred. This decides whether to test `c.state` at all.
3. **Workflow step semantics.** Can a step's code run again if the node dies after the side effect but before the step's result is saved? If so, I7 should flag only re-runs outside fault windows.
4. **Three instances on one host.** Is there a recommended way to run a three-instance cluster locally for testing?
5. **Metrics.** What format does the engine's metrics listener use, and which metrics show replica rebuild progress for F3?
6. **Existing test suites.** Depot already has a crash suite, and the storage config includes simulation-only settings. Is there a team simulation suite your findings or tests should plug into?

## Sources

- [Rivet repository and README](https://github.com/rivet-dev/rivet), main branch at commit 9e01a0a, read 1 November 2026, including `docs/general/content/benchmarking.mdx`, `engine/packages/depot-volume/README.md`, `engine/artifacts/config-schema.json` and the TypeScript and Rust examples
- [Self-hosting the control plane](https://www.rivet.dev/docs/deploy/self-host/control-plane/)
- [Control plane storage](https://rivet.dev/docs/deploy/self-host/control-plane/storage/)
- [Control plane on VMs](https://rivet.dev/docs/deploy/self-host/control-plane/vm/)
- [Control plane configuration](https://rivet.dev/docs/deploy/self-host/control-plane/configuration/)
- [Docker Compose deployment](https://rivet.dev/docs/deploy/self-host/control-plane/docker-compose/)
- Issues [#5946](https://github.com/rivet-dev/rivet/issues/5946) and [#5957](https://github.com/rivet-dev/rivet/issues/5957)
