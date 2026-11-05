# Issue draft: first placement on one instance

For rivet-dev/rivet. Not filed. Before filing: retest on the latest release, search the issues again, and push this repo so the links work.

---

**Title:** 3.0.0: a fresh three-instance cluster can bootstrap every tablet on one instance; losing that instance's disk leaves the whole cluster unavailable

## Environment

- `rivetdev/engine:3.0.0` (`sha256:5ae3c242…`), rivetkit 3.0.0.
- Three engines in Docker Compose on one host, started together with one `docker compose up`, `api_peer.advertise_address` per container on a private network, default `storage.balancing` settings, `storage.log_directory` on a per-engine volume, an S3-compatible bucket (VersityGW).
- Two nodes behind HAProxy, SQLite-backed actors.

## What the docs say

- The [VM guide](https://rivet.dev/docs/deploy/self-host/control-plane/vm/): "Start all instances within a minute of each other, so the first storage placement uses all of them."
- The [storage page](https://rivet.dev/docs/deploy/self-host/control-plane/storage/): "A write is acknowledged once a majority of the instances holding that volume have it on disk, so losing one instance loses no acknowledged write." For one instance of three or more: "Nothing is lost. After a few minutes the control plane rebuilds its replicas on the remaining instances."

## What happens

All three engines start within a second of each other. About 60 s later the coordinator bootstraps the 32 tablets on the workers it has admitted by then, and that isn't always all three:

| Workers admitted at bootstrap | Fresh clusters |
| ---: | ---: |
| 3 | 9 |
| 2 | 6 |
| 1 | 4 |

With one admitted (`tablet placement changed admitted_workers=1 desired_tablets=32 projected_tablets=32`), every tablet has a single replica on that instance. The other two are admitted about 10 s later, and learners copy the tablets over the next 10 to 70 s.

If that instance and its disk are lost in that window (killed, its log volume deleted, and a fresh instance started in its place):

- No actor answers again. Every call fails with `depot.request_deadline_exceeded`; we watched for 7.5 minutes.
- The coordinator admits all three workers but stays at `projected=32` of `desired=96`, blocked on `destination_or_quorum`, with `rivet_depot_coordinator_regrants_total{result="no_member"} 32`.
- The survivors log, once per tablet: `tablet maintainer left storage discovery and no live member can take the grant; the tablet stays unavailable until a member returns`. The fresh instance joins as a new member, so that member never returns.
- They refuse every node reconnect (`refusing envoy connect that storage could not register; the envoy reconnects`) and log `volume close will retry while the worker remains live` about 37,000 times in 9 minutes.

100 appends had been acknowledged in that window. We couldn't tell whether they survive in the bucket, because nothing could be read.

Killing the same instance without deleting its disk takes every actor down until it returns, rather than the third of them we see once storage has settled.

## Cause

`engine/packages/depot-worker/src/tablet_coordinator_machine/placement.rs:635` (unchanged on main at ac94186):

```rust
// Bootstrap on the destinations that qualify now, even when there are fewer than three.
// Local development runs a single storage worker, so waiting for a full replica set
// would leave it with no tablets at all.
if manifest.tablets.is_empty() && !self.destinations.is_empty() && self.input.discretionary
```

A worker qualifies `destination_stability_ms` (60 s) after the coordinator first sees it (`state/balancing.rs:389`). Workers that start together can still be first seen about 10 s apart: in the single-worker clusters, the other two were admitted 9 to 10 s after the first. The first to qualify then bootstraps alone. Starting within a minute isn't enough; they would have to be first seen within the same coordinator turn.

## Retest on 3.0.5

A maintainer on Discord had just fixed an availability-zone bug that could allow this, and asked for a retest. 3.0.5 contains that fix (465e5e163). On 5 November, with `rivetdev/engine:3.0.5` (`sha256:c0f18c3b…`), the first fresh cluster placed all 32 tablets on one instance (`admitted_workers=1`, 63 s after start). After that instance and its disk were destroyed, all 20 ledgers stayed unavailable for the 5 minutes we watched, with the same signs as on 3.0.0: placement at 32 of 96, `blocked=destination_or_quorum`, 32 `no_member` re-grants, and `no live member can take the grant` once per tablet. Logs and metrics are in `results/probes/early-placement-3.0.5/`.

The fix doesn't reach this case: with no zone labels configured, `require_zone_labels` stays false, and `placement.rs` still bootstraps on whatever workers qualify first.

## Repro

`repro/early-placement/repro.py` in https://github.com/sohamghodake/Rivet-chaos (`ENGINE_IMAGE=rivetdev/engine:3.0.5 repro/early-placement/repro.py` for 3.0.5) starts fresh clusters until one bootstraps on a single instance, writes, destroys that instance and its disk, and reads everything back. It caught the single-instance case on its second cluster. Logs and metrics from that run are in `results/probes/early-placement-1793825583/`.

## Possible fixes

- Wait before bootstrapping until the set of qualifying workers has stopped growing for a while, or until a configured expected size is reached, keeping today's behaviour for a single worker.
- Or replicate to the late workers before treating the cluster as ready, and say in the docs how to tell when that has happened (`rivet_depot_coordinator_placement{kind="projected"}` reaching `desired` works today).
- At least, change the VM guide's sentence: starting together doesn't guarantee the first placement uses every instance.

We wait for `projected == desired` with every worker admitted before injecting faults (`cluster/settle.sh`).
