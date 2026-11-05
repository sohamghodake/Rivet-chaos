# instance-down-long-s3-1793829520

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-down-long` |
| Seed | 3 |
| Started | 2026-11-04 22:00:41 UTC |
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
| I1 | Durability | PASS | 12244 | 0 |
| I2 | No phantom writes | PASS | 12364 | 0 |
| I3 | No duplicates | PASS | 12364 | 0 |
| I4 | Real-time visibility | PASS | 1379 | 0 |
| I5 | Recovery | PASS | 50 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1511 | 0 |
| I8 | Alarms | PASS | 770 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-1 | 60.0 s | 245.0 s | 184.9 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 50 | 27 | -141.1 s | 6.4 s | 191.1 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6323 | 6323 | 0 | 0 | 100.00% | 9.2 ms | 57.9 ms | 239.5 ms |
| During faults | 1089 | 574 | 0 | 515 | 52.71% | 9.1 ms | 2034.7 ms | 4788.2 ms |
| After faults | 5349 | 5347 | 0 | 2 | 99.96% | 7.9 ms | 18.3 ms | 4718.6 ms |
| All | 12761 | 12244 | 0 | 517 | 95.95% | 8.5 ms | 50.4 ms | 4788.2 ms |

Reads during the load: 1329 succeeded out of 1387.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 848 turns: 752 acknowledged, 96 with an unknown outcome, 0 failed.

The sink recorded 1511 tool calls for 1511 distinct steps.

770 alarms fired, after their due time by p50 8.1 ms and at most 190543.1 ms.

## Errors

| Count | Error |
| ---: | --- |
| 633 | `timed out after 5000 ms` |
| 19 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 16 | `queue send failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_m` |
| 1 | `action failed: 502 Bad Gateway` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=38&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=44&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
