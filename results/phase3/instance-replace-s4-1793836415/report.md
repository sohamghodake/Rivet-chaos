# instance-replace-s4-1793836415

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-replace` |
| Seed | 4 |
| Started | 2026-11-04 23:55:24 UTC |
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
| I1 | Durability | PASS | 41120 | 0 |
| I2 | No phantom writes | PASS | 41186 | 0 |
| I3 | No duplicates | PASS | 41186 | 0 |
| I4 | Real-time visibility | PASS | 4500 | 0 |
| I5 | Recovery | PASS | 53 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 2587 | 0 |
| I8 | Alarms | PASS | 1300 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | replace | engine-1 | 60.0 s | 93.8 s | 33.8 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 53 | 4 | 3.2 s | 5.5 s | 38.6 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6511 | 6511 | 0 | 0 | 100.00% | 7.2 ms | 13.9 ms | 23.6 ms |
| During faults | 121 | 23 | 0 | 98 | 19.01% | 3362.8 ms | 4993.0 ms | 4993.0 ms |
| After faults | 34586 | 34586 | 0 | 0 | 100.00% | 7.7 ms | 24.0 ms | 3059.7 ms |
| All | 41218 | 41120 | 0 | 98 | 99.76% | 7.6 ms | 23.0 ms | 4993.0 ms |

Reads during the load: 4450 succeeded out of 4453.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 1323 turns: 1283 acknowledged, 40 with an unknown outcome, 0 failed.

The sink recorded 2587 tool calls for 2587 distinct steps.

1300 alarms fired, after their due time by p50 7.5 ms and at most 36808.1 ms.

## Errors

| Count | Error |
| ---: | --- |
| 136 | `timed out after 5000 ms` |
| 3 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=12&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=17&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
