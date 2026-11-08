# instance-pause-s3-1794154327

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-pause` |
| Seed | 3 |
| Started | 2026-11-08 16:14:03 UTC |
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
| I1 | Durability | PASS | 22521 | 0 |
| I2 | No phantom writes | PASS | 22562 | 0 |
| I3 | No duplicates | PASS | 22562 | 0 |
| I4 | Real-time visibility | PASS | 2481 | 0 |
| I5 | Recovery | PASS | 82 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1537 | 0 |
| I8 | Alarms | PASS | 785 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | pause | engine-1 | 60.1 s | 105.2 s | 45.1 s |
| 2 | pause | engine-2 | 180.1 s | 225.2 s | 45.2 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 39 | 16 | 1.2 s | 5.4 s | 48.9 s |
| 2 | 43 | 16 | 0.5 s | 6.3 s | 49.3 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6461 | 6461 | 0 | 0 | 100.00% | 7.7 ms | 24.2 ms | 47.3 ms |
| During faults | 453 | 226 | 0 | 227 | 49.89% | 8.9 ms | 4898.8 ms | 4993.0 ms |
| After faults | 15836 | 15834 | 0 | 2 | 99.99% | 7.7 ms | 31.0 ms | 1216.5 ms |
| All | 22750 | 22521 | 0 | 229 | 98.99% | 7.7 ms | 31.9 ms | 4993.0 ms |

Reads during the load: 2431 succeeded out of 2461.

Final reads: 50 succeeded out of 50 attempts, over 0.7 s.

## Sessions

10 sessions were sent 854 turns: 765 acknowledged, 89 with an unknown outcome, 0 failed.

The sink recorded 1537 tool calls for 1537 distinct steps.

785 alarms fired, after their due time by p50 7.6 ms and at most 47006.2 ms.

## Errors

| Count | Error |
| ---: | --- |
| 337 | `timed out after 5000 ms` |
| 5 | `action failed (ups/transport): Worker transport failed; the operation may have executed., metadata=None` |
| 3 | `queue send failed (ups/transport): Worker transport failed; the operation may have executed., metadata=None` |
| 2 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 1 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
