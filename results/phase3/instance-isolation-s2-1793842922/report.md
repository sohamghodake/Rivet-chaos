# instance-isolation-s2-1793842922

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `instance-isolation` |
| Seed | 2 |
| Started | 2026-11-05 01:44:01 UTC |
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
| I1 | Durability | PASS | 19134 | 0 |
| I2 | No phantom writes | PASS | 19180 | 0 |
| I3 | No duplicates | PASS | 19180 | 0 |
| I4 | Real-time visibility | PASS | 2161 | 0 |
| I5 | Recovery | PASS | 113 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | FAIL | 1401 | 1 |
| I8 | Alarms | PASS | 718 | 0 |

### I7 violations

- tool call 8/28/tool-1 ran 2 times, from 120.9 s, with no fault during its step

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `peer` | engine-3 | 60.0 s | 120.6 s | 60.6 s |
| 2 | disconnect from `peer` | engine-3 | 210.0 s | 270.5 s | 60.5 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 59 | 42 | -13.5 s | 4.8 s | 65.2 s |
| 2 | 54 | 44 | -29.8 s | 1.4 s | 61.7 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6444 | 6444 | 0 | 0 | 100.00% | 7.4 ms | 15.1 ms | 84.6 ms |
| During faults | 751 | 422 | 0 | 329 | 56.19% | 8.5 ms | 4102.1 ms | 4784.1 ms |
| After faults | 12268 | 12268 | 0 | 0 | 100.00% | 7.9 ms | 28.7 ms | 4493.3 ms |
| All | 19463 | 19134 | 0 | 329 | 98.31% | 7.7 ms | 28.0 ms | 4784.1 ms |

Reads during the load: 2111 succeeded out of 2145.

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Sessions

10 sessions were sent 816 turns: 704 acknowledged, 112 with an unknown outcome, 0 failed.

The sink recorded 1402 tool calls for 1401 distinct steps.

718 alarms fired, after their due time by p50 7.4 ms and at most 2620.5 ms.

## Errors

| Count | Error |
| ---: | --- |
| 442 | `timed out after 5000 ms` |
| 24 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 9 | `queue send failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_m` |
