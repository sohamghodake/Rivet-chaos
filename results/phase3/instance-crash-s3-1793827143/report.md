# instance-crash-s3-1793827143

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-crash` |
| Seed | 3 |
| Started | 2026-11-04 21:21:03 UTC |
| Engine image | `rivetdev/engine:3.0.0@sha256:5ae3c24275eeff87ad1962f4d66fcdb85786fc1397feeeb55afc2ca1f7ccf6f2` |
| rivet-chaos commit | 5740a28 |
| rivetkit | 3.0.0 |
| rivetkit-client | 3.0.0 |
| CPUs | 16 |
| Docker | 29.0.1 on Docker Desktop, kernel 6.6.87.2-microsoft-standard-WSL2 |
| Memory | 7.4 GiB |

## Invariants

| ID | Invariant | Result | Checked | Violations |
| --- | --- | --- | ---: | ---: |
| I1 | Durability | PASS | 24173 | 0 |
| I2 | No phantom writes | PASS | 24228 | 0 |
| I3 | No duplicates | PASS | 24228 | 0 |
| I4 | Real-time visibility | PASS | 2704 | 0 |
| I5 | Recovery | PASS | 92 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1680 | 0 |
| I8 | Alarms | PASS | 858 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-1 | 60.0 s | 93.9 s | 33.9 s |
| 2 | kill | engine-2 | 180.0 s | 213.9 s | 33.8 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 51 | 16 | 2.9 s | 5.7 s | 38.5 s |
| 2 | 41 | 8 | 3.7 s | 5.1 s | 38.7 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6416 | 6416 | 0 | 0 | 100.00% | 8.2 ms | 29.2 ms | 74.4 ms |
| During faults | 346 | 145 | 0 | 201 | 41.91% | 9.4 ms | 4948.0 ms | 5001.2 ms |
| After faults | 17612 | 17612 | 0 | 0 | 100.00% | 8.1 ms | 38.6 ms | 3471.4 ms |
| All | 24374 | 24173 | 0 | 201 | 99.18% | 8.1 ms | 39.8 ms | 5001.2 ms |

Reads during the load: 2654 succeeded out of 2675.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 904 turns: 842 acknowledged, 62 with an unknown outcome, 0 failed.

The sink recorded 1680 tool calls for 1680 distinct steps.

858 alarms fired, after their due time by p50 7.8 ms and at most 37137.0 ms.

## Errors

| Count | Error |
| ---: | --- |
| 225 | `timed out after 5000 ms` |
| 36 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 16 | `queue send failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_m` |
| 2 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=6&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client ` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (ups_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("ups_endpoint_unresolved"), "retry_after_ms": Numb` |
| 1 | `action failed: 502 Bad Gateway` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=28&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=3&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client ` |
| 1 | `queue send failed (sqlite/transaction_closed): An internal error occurred, metadata=None` |
