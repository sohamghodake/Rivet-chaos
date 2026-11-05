# instance-replace-s2-1793835180

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-replace` |
| Seed | 2 |
| Started | 2026-11-04 23:34:57 UTC |
| Engine image | `rivetdev/engine:3.0.0@sha256:5ae3c24275eeff87ad1962f4d66fcdb85786fc1397feeeb55afc2ca1f7ccf6f2` |
| rivet-chaos commit | 9f5be2f |
| rivetkit | 3.0.0 |
| rivetkit-client | 3.0.0 |
| CPUs | 16 |
| Docker | 29.0.1 on Docker Desktop, kernel 6.6.87.2-microsoft-standard-WSL2 |
| Memory | 7.4 GiB |

## Invariants

| ID | Invariant | Result | Checked | Violations |
| --- | --- | --- | ---: | ---: |
| I1 | Durability | PASS | 41825 | 0 |
| I2 | No phantom writes | PASS | 41826 | 0 |
| I3 | No duplicates | PASS | 41826 | 0 |
| I4 | Real-time visibility | PASS | 4713 | 0 |
| I5 | Recovery | PASS | 27 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 2623 | 0 |
| I8 | Alarms | PASS | 1329 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | replace | engine-3 | 60.0 s | 94.4 s | 34.4 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 27 | 25 | -2.9 s | 1.8 s | 34.9 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6456 | 6456 | 0 | 0 | 100.00% | 7.2 ms | 13.4 ms | 107.9 ms |
| During faults | 620 | 523 | 0 | 97 | 84.35% | 7.3 ms | 1147.9 ms | 4812.8 ms |
| After faults | 34846 | 34846 | 0 | 0 | 100.00% | 7.8 ms | 23.9 ms | 140.4 ms |
| All | 41922 | 41825 | 0 | 97 | 99.77% | 7.7 ms | 23.2 ms | 4812.8 ms |

Reads during the load: 4663 succeeded out of 4669.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 1361 turns: 1326 acknowledged, 35 with an unknown outcome, 0 failed.

The sink recorded 2623 tool calls for 2623 distinct steps.

1329 alarms fired, after their due time by p50 7.4 ms and at most 75.8 ms.

## Errors

| Count | Error |
| ---: | --- |
| 78 | `timed out after 5000 ms` |
| 44 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 16 | `queue send failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_m` |
