# A fresh cluster can keep every tablet on one instance

Rivet's docs say:

- [VM guide](https://rivet.dev/docs/deploy/self-host/control-plane/vm/): "Start all instances within a minute of each other, so the first storage placement uses all of them."
- [Storage](https://rivet.dev/docs/deploy/self-host/control-plane/storage/): "A write is acknowledged once a majority of the instances holding that volume have it on disk, so losing one instance loses no acknowledged write." Losing one instance of three or more: "Nothing is lost. After a few minutes the control plane rebuilds its replicas on the remaining instances."

On Rivet 3.0.0, with all three engines started within a second of each other, the first placement doesn't always use all of them. About 60 s after start, the coordinator places the cluster's 32 tablets on the workers it has admitted so far:

| Workers admitted at the first placement | Fresh clusters |
| ---: | ---: |
| 3 | 9 |
| 2 | 6 |
| 1 | 4 |

The 19 clusters are this repo's runs from 3 and 4 November, each started with `cluster/up.sh` or this script. With one worker admitted, every tablet's only replica is on that instance; the others are admitted about 10 s later, and the copies finish between 10 and 70 s after that.

## What losing that instance does

`repro.py` starts fresh clusters until one places every tablet on a single instance, acknowledges 100 appends across 20 ledgers, then destroys that instance and its log volume and starts a fresh one in its place, as when a machine and its disk are replaced. On 4 November it caught the single-instance placement on its second cluster, and then:

- None of the 20 ledgers answered within 5 minutes, or when tried again 7.5 minutes after the instance was destroyed. Every call failed with `depot.request_deadline_exceeded`.
- The coordinator admitted all three workers again but stayed at 32 of 96 replicas placed, `blocked=destination_or_quorum`, with 32 re-grants that found no member (`rivet_depot_coordinator_regrants_total{result="no_member"} 32`).
- The survivors logged, once per tablet: `tablet maintainer left storage discovery and no live member can take the grant; the tablet stays unavailable until a member returns`. The member they wait for was on the destroyed disk, and the fresh instance joins as a new member, so it never returns.
- They also refused every node reconnect (`refusing envoy connect that storage could not register`) and logged `volume close will retry while the worker remains live` 36,863 times before the cluster was torn down, about 9 minutes later.

Whether the 100 acknowledged appends survive in the bucket couldn't be checked, because nothing could be read. The whole cluster, not one instance's share, stayed down.

Killing the instance without deleting its disk (`results/probes/early-kill-*`) also took every actor down until it returned, and afterwards placement stayed at 46 of 96 replicas for at least five minutes.

## Running it

```bash
repro/early-placement/repro.py
```

To run it against another release, set `ENGINE_IMAGE`, such as `ENGINE_IMAGE=rivetdev/engine:3.0.5 repro/early-placement/repro.py`. On 5 November it reproduced on 3.0.5 on the first cluster, with the same outcome as on 3.0.0 (`results/probes/early-placement-3.0.5/`). 3.0.5 includes a placement fix for availability zones, which doesn't apply when no zones are configured.

It needs Docker with Compose, `curl`, `openssl` and Python 3, and starts up to `ATTEMPTS` (8) fresh clusters, about two minutes each, until one places every tablet on one instance. It leaves the cluster running for its logs; `cluster/down.sh` removes it. The run above is archived in `results/probes/early-placement-1793825583/`.

The harness avoids this window: `cluster/up.sh` runs `cluster/settle.sh`, which waits until the coordinator has placed every replica on all three instances before anything is faulted.
