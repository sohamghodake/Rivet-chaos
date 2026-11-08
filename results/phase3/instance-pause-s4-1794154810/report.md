# instance-pause-s4-1794154810

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-pause` |
| Seed | 4 |
| Started | 2026-11-08 16:21:25 UTC |
| Engine image | `rivetdev/engine:3.0.0@sha256:5ae3c24275eeff87ad1962f4d66fcdb85786fc1397feeeb55afc2ca1f7ccf6f2` |
| rivet-chaos commit | 6388a11 |
| rivetkit | 3.0.0 |
| rivetkit-client | 3.0.0 |
| CPUs | 16 |
| Docker | 29.0.1 on Docker Desktop, kernel 6.6.87.2-microsoft-standard-WSL2 |
| Memory | 7.4 GiB |

## Invariants

| ID | Invariant | Result | Checked | Violations |
| --- | --- | --- | ---: | ---: |
| I1 | Durability | PASS | 23218 | 0 |
| I2 | No phantom writes | PASS | 23291 | 0 |
| I3 | No duplicates | PASS | 23291 | 0 |
| I4 | Real-time visibility | PASS | 2524 | 0 |
| I5 | Recovery | PASS | 82 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1633 | 0 |
| I8 | Alarms | PASS | 831 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | pause | engine-1 | 60.1 s | 105.3 s | 45.1 s |
| 2 | pause | engine-3 | 180.1 s | 225.2 s | 45.1 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 28 | 28 | -7.6 s | -3.6 s | 41.4 s |
| 2 | 54 | 0 | 1.3 s | 4.8 s | 47.8 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6455 | 6455 | 0 | 0 | 100.00% | 8.0 ms | 22.7 ms | 61.5 ms |
| During faults | 1007 | 779 | 0 | 228 | 77.36% | 7.4 ms | 2789.4 ms | 4816.9 ms |
| After faults | 15984 | 15984 | 0 | 0 | 100.00% | 7.8 ms | 31.9 ms | 769.0 ms |
| All | 23446 | 23218 | 0 | 228 | 99.03% | 7.8 ms | 33.8 ms | 4816.9 ms |

Reads during the load: 2474 succeeded out of 2498.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 875 turns: 803 acknowledged, 72 with an unknown outcome, 0 failed.

The sink recorded 1633 tool calls for 1633 distinct steps.

831 alarms fired, after their due time by p50 7.5 ms and at most 47972.1 ms.

## Errors

| Count | Error |
| ---: | --- |
| 316 | `timed out after 5000 ms` |
| 3 | `action failed (ups/transport): Worker transport failed; the operation may have executed., metadata=None` |
| 2 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 1 | `action failed (core/internal_error): An internal error occurred, metadata=Some(Object {"error": String("actor response aborted")})` |
| 1 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 1 | `queue send failed (ups/transport): Worker transport failed; the operation may have executed., metadata=None` |
