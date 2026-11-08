# bucket-outage-s3-1794156175

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `bucket-outage` |
| Seed | 3 |
| Started | 2026-11-08 16:44:12 UTC |
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
| I1 | Durability | PASS | 18016 | 0 |
| I2 | No phantom writes | PASS | 18018 | 0 |
| I3 | No duplicates | PASS | 18018 | 0 |
| I4 | Real-time visibility | PASS | 2080 | 0 |
| I5 | Recovery | PASS | 60 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1312 | 0 |
| I8 | Alarms | PASS | 656 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `storage` | s3 | 60.1 s | 181.8 s | 121.7 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 60 | 0 | 11.0 s | 13.4 s | 106.3 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 5888 | 5888 | 0 | 0 | 100.00% | 11.3 ms | 119.5 ms | 257.7 ms |
| During faults | 12903 | 2942 | 0 | 9961 | 22.80% | 9.9 ms | 37.8 ms | 81.9 ms |
| After faults | 9390 | 9186 | 0 | 204 | 97.83% | 28.5 ms | 255.6 ms | 3502.1 ms |
| All | 28181 | 18016 | 0 | 10165 | 63.93% | 21.6 ms | 179.6 ms | 3502.1 ms |

Reads during the load: 2029 succeeded out of 3140.

Final reads: 51 succeeded out of 51 attempts, over 31.1 s.

## Sessions

10 sessions were sent 977 turns: 656 acknowledged, 321 with an unknown outcome, 0 failed.

The sink recorded 1312 tool calls for 1312 distinct steps.

656 alarms fired, after their due time by p50 21.2 ms and at most 106750.1 ms.

## Errors

| Count | Error |
| ---: | --- |
| 11205 | `action failed: 503 Service Unavailable` |
| 306 | `queue send failed: 503 Service Unavailable` |
| 40 | `timed out after 5000 ms` |
| 15 | `action failed (depot/unavailable): The storage host could not finish the command (command_capacity); its outcome may be unknown., metadata=Some(Object {"reason": String("command_capacity")})` |
| 8 | `action failed: 502 Bad Gateway` |
| 5 | `action failed (core/internal_error): An internal error occurred, metadata=Some(Object {"error": String("actor response aborted")})` |
| 5 | `queue send failed (depot/unavailable): The storage host could not finish the command (command_capacity); its outcome may be unknown., metadata=Some(Object {"reason": String("command_capacity")})` |
| 3 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 2 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=31&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 2 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=40&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=19&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=28&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=41&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=44&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `queue send failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 1 | `queue send failed: 502 Bad Gateway` |
