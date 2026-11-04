# instance-down-long-s1-1793828581

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-down-long` |
| Seed | 1 |
| Started | 2026-11-04 21:45:02 UTC |
| Engine image | `rivetdev/engine:3.0.0@sha256:5ae3c24275eeff87ad1962f4d66fcdb85786fc1397feeeb55afc2ca1f7ccf6f2` |
| rivet-chaos commit | 5d29f6d |
| rivetkit | 3.0.0 |
| rivetkit-client | 3.0.0 |
| CPUs | 16 |
| Docker | 29.0.1 on Docker Desktop, kernel 6.6.87.2-microsoft-standard-WSL2 |
| Memory | 7.4 GiB |

## Invariants

| ID | Invariant | Result | Checked | Violations |
| --- | --- | --- | ---: | ---: |
| I1 | Durability | PASS | 11993 | 0 |
| I2 | No phantom writes | PASS | 12128 | 0 |
| I3 | No duplicates | PASS | 12128 | 0 |
| I4 | Real-time visibility | PASS | 1334 | 0 |
| I5 | Recovery | PASS | 59 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1164 | 0 |
| I8 | Alarms | PASS | 606 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-1 | 60.0 s | 244.3 s | 184.3 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 59 | 25 | 6.2 s | 9.8 s | 193.5 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6475 | 6475 | 0 | 0 | 100.00% | 8.7 ms | 21.7 ms | 35.6 ms |
| During faults | 850 | 323 | 0 | 527 | 38.00% | 9.2 ms | 1964.0 ms | 4988.9 ms |
| After faults | 5197 | 5195 | 0 | 2 | 99.96% | 9.1 ms | 68.5 ms | 4649.0 ms |
| All | 12522 | 11993 | 0 | 529 | 95.78% | 8.9 ms | 34.8 ms | 4988.9 ms |

Reads during the load: 1284 succeeded out of 1340.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 733 turns: 574 acknowledged, 159 with an unknown outcome, 0 failed.

The sink recorded 1164 tool calls for 1164 distinct steps.

606 alarms fired, after their due time by p50 8.3 ms and at most 36372.0 ms.

## Errors

| Count | Error |
| ---: | --- |
| 713 | `timed out after 5000 ms` |
| 15 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 12 | `queue send failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_m` |
| 2 | `action failed: 502 Bad Gateway` |
| 1 | `action failed (ups/transport): Worker transport failed; the operation may have executed., metadata=None` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=24&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
