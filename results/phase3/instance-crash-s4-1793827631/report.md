# instance-crash-s4-1793827631

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-crash` |
| Seed | 4 |
| Started | 2026-11-04 21:29:07 UTC |
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
| I1 | Durability | PASS | 23892 | 0 |
| I2 | No phantom writes | PASS | 23998 | 0 |
| I3 | No duplicates | PASS | 23998 | 0 |
| I4 | Real-time visibility | PASS | 2593 | 0 |
| I5 | Recovery | PASS | 81 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1703 | 0 |
| I8 | Alarms | PASS | 873 | 0 |

I7: 1 tool calls ran more than once, each with a fault during its step.

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-1 | 60.0 s | 93.7 s | 33.6 s |
| 2 | kill | engine-3 | 180.0 s | 214.1 s | 34.1 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 40 | 9 | 4.7 s | 7.6 s | 40.8 s |
| 2 | 41 | 12 | 2.0 s | 8.2 s | 42.0 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6430 | 6430 | 0 | 0 | 100.00% | 8.0 ms | 32.6 ms | 99.6 ms |
| During faults | 390 | 184 | 0 | 206 | 47.18% | 9.1 ms | 4976.6 ms | 4984.8 ms |
| After faults | 17282 | 17278 | 0 | 4 | 99.98% | 8.3 ms | 39.2 ms | 4751.4 ms |
| All | 24102 | 23892 | 0 | 210 | 99.13% | 8.2 ms | 40.3 ms | 4984.8 ms |

Reads during the load: 2543 succeeded out of 2569.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 900 turns: 841 acknowledged, 59 with an unknown outcome, 0 failed.

The sink recorded 1704 tool calls for 1703 distinct steps.

873 alarms fired, after their due time by p50 7.8 ms and at most 37778.9 ms.

## Errors

| Count | Error |
| ---: | --- |
| 265 | `timed out after 5000 ms` |
| 13 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 6 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 3 | `action failed: 502 Bad Gateway` |
| 2 | `action failed (depot/overloaded): The storage host refused the command (endpoint_unresolved); retry after 100 ms., metadata=None` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=34&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=5&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=9&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/read?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=5&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client er` |
| 1 | `queue send failed (core/internal_error): An internal error occurred, metadata=Some(Object {"error": String("actor response aborted")})` |
| 1 | `queue send failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_m` |
