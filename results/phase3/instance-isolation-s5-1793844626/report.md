# instance-isolation-s5-1793844626

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-isolation` |
| Seed | 5 |
| Started | 2026-11-05 02:12:25 UTC |
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
| I1 | Durability | PASS | 17887 | 0 |
| I2 | No phantom writes | PASS | 17994 | 0 |
| I3 | No duplicates | PASS | 17994 | 0 |
| I4 | Real-time visibility | PASS | 2040 | 0 |
| I5 | Recovery | PASS | 114 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1481 | 0 |
| I8 | Alarms | PASS | 757 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `peer` | engine-3 | 60.0 s | 120.4 s | 60.4 s |
| 2 | disconnect from `peer` | engine-2 | 210.0 s | 270.4 s | 60.4 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 55 | 35 | -19.1 s | 1.4 s | 61.2 s |
| 2 | 59 | 23 | 0.3 s | 5.8 s | 65.8 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6479 | 6479 | 0 | 0 | 100.00% | 7.2 ms | 13.6 ms | 21.2 ms |
| During faults | 561 | 238 | 0 | 323 | 42.42% | 8.0 ms | 4747.3 ms | 4866.0 ms |
| After faults | 11210 | 11170 | 0 | 40 | 99.64% | 7.3 ms | 19.2 ms | 3500.0 ms |
| All | 18250 | 17887 | 0 | 363 | 98.01% | 7.3 ms | 17.1 ms | 4866.0 ms |

Reads during the load: 1990 succeeded out of 2032.

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Sessions

10 sessions were sent 838 turns: 747 acknowledged, 91 with an unknown outcome, 0 failed.

The sink recorded 1481 tool calls for 1481 distinct steps.

757 alarms fired, after their due time by p50 6.9 ms and at most 2841.3 ms.

## Errors

| Count | Error |
| ---: | --- |
| 478 | `timed out after 5000 ms` |
| 11 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 5 | `queue send failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_m` |
| 1 | `action failed (depot/request_deadline_exceeded): The command deadline elapsed; its outcome may be unknown., metadata=None` |
| 1 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
