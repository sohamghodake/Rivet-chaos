# instance-replace-s5-1793837026

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-replace` |
| Seed | 5 |
| Started | 2026-11-05 00:05:43 UTC |
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
| I1 | Durability | PASS | 41454 | 0 |
| I2 | No phantom writes | PASS | 41461 | 0 |
| I3 | No duplicates | PASS | 41461 | 0 |
| I4 | Real-time visibility | PASS | 4657 | 0 |
| I5 | Recovery | PASS | 26 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 2702 | 0 |
| I8 | Alarms | PASS | 1364 | 0 |

I8: 1 alarms fired on time by the wall clock but early by the monotonic clock, as a forward wall-clock adjustment causes.

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | replace | engine-3 | 60.0 s | 94.4 s | 34.4 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 26 | 24 | -2.8 s | 5.1 s | 33.8 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6477 | 6477 | 0 | 0 | 100.00% | 7.3 ms | 13.9 ms | 109.8 ms |
| During faults | 442 | 342 | 0 | 100 | 77.38% | 7.6 ms | 2041.9 ms | 4132.9 ms |
| After faults | 34635 | 34635 | 0 | 0 | 100.00% | 7.9 ms | 23.2 ms | 3616.8 ms |
| All | 41554 | 41454 | 0 | 100 | 99.76% | 7.8 ms | 22.7 ms | 4132.9 ms |

Reads during the load: 4607 succeeded out of 4616.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 1373 turns: 1360 acknowledged, 13 with an unknown outcome, 0 failed.

The sink recorded 2702 tool calls for 2702 distinct steps.

1364 alarms fired, after their due time by p50 7.4 ms and at most 75.5 ms.

## Errors

| Count | Error |
| ---: | --- |
| 77 | `timed out after 5000 ms` |
| 38 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 5 | `queue send failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_m` |
| 1 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=15&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
