# Phase 1 findings

Written 1 November 2026, against Rivet 3.0.0 and source commit 9e01a0a.

## Exit criteria

| Criterion | Result |
| --- | --- |
| 1,000 appends read back exactly | Pass. p50 about 4.5 ms, p99 about 7 ms |
| One agent session completes 10 turns | Pass |
| The sink shows one effect per tool step | Pass. 21 and 22 tool steps in two of the logged runs |

`make phase1` ran 5 times with all checks passing. Three deliberately broken versions of the actors each made it fail: a tool step that ran twice, a read that lost a row, and duplicates not reported.

These ran on one engine binary from npm, with host processes rather than containers, on 16 vCPUs and 7.4 GiB of RAM under WSL2, using a debug build of the client. They show the workload works, and they aren't benchmark numbers.

`cluster/compose.single.yaml` was verified on 3 November, once Docker was connected: the engine came up, the pool was created, and the same check passed against the container.

## Changes the plan needs

| Plan says | Reality | What was done |
| --- | --- | --- |
| `rivetdev/engine:next`; "Rivet 3.0 is still in preview" | 3.0.0 final came out on 1 November at about 11:05 UTC, after `next` was last built | Pinned the image to `3.0.0` by digest, and `rivetkit` and `rivetkit-client` to `3.0.0` |
| `depot.log_directory` | The key is `storage.log_directory`; `depot` is rejected | Used `storage`; phase 2's engine configs must too |
| `c.schedule.after(...)` inside the workflow | Workflow steps have no `schedule` | `createVars` captures `c.schedule`, and the step calls it through `step.vars` |
| I8: each alarm fires once | The step booking the alarm can rerun after a crash (see question 3), which books a second alarm | Applied in phase 3. A booking that fires twice always fails I8; a booking step that ran twice is allowed only if a fault overlapped it, as for I7 ([phase 3 findings](phase3-findings.md#decisions-worth-knowing)) |
| `bootstrap.sh` waits for `/health`, then creates the pool | `/health` can pass while pool creation still fails with `depot/overloaded` (`routing_unavailable`) | `bootstrap.sh` retries pool creation |

The plan's fallback of calling actions over HTTP isn't needed: the Rust client handled actions, queues and WebSocket events on 3.0.0.

## Open questions

### 1. Crash policy: answered well enough to proceed

The `Destroy` default doesn't appear to decide what happens to these actors.

- The gateway rejects any policy other than `sleep` (`engine/packages/pegboard-gateway/src/routes.rs:316`).
- The client's `get_or_create` path sends `sleep` (`rivetkit-rust/packages/client/src/remote_manager.rs:690`).
- Outside tests and API types, a source search found no other engine code that reads the field. This was a search, not a full read of the engine.
- The client's direct `PUT /actors` path still sends `destroy` (`remote_manager.rs:343`, `:408`). The driver should keep using `get_or_create`.

One probe supports this: an actor with 200 rows had its node killed with `SIGKILL`, with a second node already running. The next read returned all 200 rows after 44 s, with no client errors, because the gateway held the request open. That's a single sample on a single engine, so F4 in phase 3 is the real test. It also means I5 should expect tens of seconds for a node crash.

### 2. State durability: answered

`c.state` isn't durable when an action returns. The state docs say saves are throttled (`stateSaveInterval`, 1 second by default) and not tied to action boundaries, unless the action awaits `c.saveState({ immediate: true })`.

**Decision:** the workload keeps its durable data in SQLite and doesn't use `c.state`, so I1 tests SQLite only.

### 3. Workflow step semantics: answered from the code

A step's code can run again. `executeStep` runs the step (`rivetkit-typescript/packages/workflow-engine/src/context.ts:921`) and only then saves its result (`:945`), so a crash between the two reruns it. Steps are at-least-once, and I7 should flag reruns only outside fault windows, as the plan suspected.

The attempt counter is also only saved at that later point, so a crash inside a step doesn't appear to use up a retry. F8 should confirm this with repeated kills.

### 4. Three instances on one host: still open

The docs cover one instance in Compose, and three or more on VMs or Kubernetes. Ask on Discord (draft below).

### 5. Metrics: mostly answered

The engine serves Prometheus text format at `:6430/metrics`, confirmed on a running engine. For F3, the source defines learner metrics in `engine/packages/depot-worker/src/metrics.rs`, including `depot_learner_catch_up_blocked_total`, but none appeared on an idle single instance. Check that they appear during a real rebuild in phase 3.

### 6. Existing test suites: partly answered

`engine/packages/depot-volume/README.md` describes a crash suite that kills processes at each append boundary and excludes power loss and torn writes. The storage config also has simulation-only settings such as `simulated_fs_read_cost_us`. Whether there's a team suite your tests should plug into isn't clear from the code, so that's on the Discord draft too.

## Upstream candidates

Nothing has been filed. Before filing, retest on the latest release and search the issues again.

**1. The self-hosting docs give a config the engine refuses to load (confirmed; fixed upstream on 3 November in rivet-dev/website commit 6417f2d, #173).** The [Docker Compose page](https://rivet.dev/docs/deploy/self-host/control-plane/docker-compose/) uses `depot` and `auto_provision_tablets`. Both 3.0.0 and 3.0.0-preview.6 (the npm build matching the `next` tag) exit at startup:

```text
Error: failed to deserialize config
Caused by:
    unknown field `depot`, expected one of `auth`, `guard`, `storage`, ...
```

Renaming only the top-level key then fails on `auto_provision_tablets`. Using `storage` and `auto_provision_partitions` works on both builds. The cause is `#[serde(rename = "storage")]` at `engine/packages/config/src/config/mod.rs:66`.

GitHub issue searches for `auto_provision_tablets` and `"unknown field" depot` found no report of this. The docs source wasn't located, and the search of the repo for it timed out, so find it before opening a PR. On 3 November the `rivetdev/engine:next` image (by then 3.0.1-preview.2) rejected the config the same way. It goes further than this one page; see [phase 2 findings](phase2-findings.md).

**2. The `rivetkit-client` README doesn't match the 3.0.0 crate (confirmed).** It uses `TransportKind::Ws` (the variant is `WebSocket`, so this fails to compile), `Client::new(...)?` (it returns `Self`) and `rivetkit-client = "0.1.0"`.

## Worth watching

These aren't bugs so far, but they could matter once faults are running:

- **`UPS buffer is near full or full`** (`stream_packets`, 16 of 16), logged 7 times in one run with a single client.
- **`durable alarm delivery is for a revision that is no longer armed`**, logged once just before the engine stopped, followed by a retry. Look for it if I8 fails.

## Discord draft

The draft is in [discord-draft.md](discord-draft.md), with the questions phase 2 added.
