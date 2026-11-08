# bucket-outage-s5-1794157088

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `bucket-outage` |
| Seed | 5 |
| Started | 2026-11-08 16:59:29 UTC |
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
| I1 | Durability | PASS | 16238 | 0 |
| I2 | No phantom writes | PASS | 16243 | 0 |
| I3 | No duplicates | PASS | 16243 | 0 |
| I4 | Real-time visibility | PASS | 1844 | 0 |
| I5 | Recovery | PASS | 60 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1226 | 0 |
| I8 | Alarms | FAIL | 616 | 1 |

### I8 violations

- alarm 1/83 fired 17 ms before its wall-clock due time

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `storage` | s3 | 60.0 s | 182.7 s | 122.7 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 60 | 0 | 19.3 s | 21.9 s | 118.2 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 5769 | 5769 | 0 | 0 | 100.00% | 21.6 ms | 75.1 ms | 157.2 ms |
| During faults | 12601 | 2393 | 0 | 10208 | 18.99% | 25.2 ms | 176.4 ms | 423.9 ms |
| After faults | 8468 | 8076 | 0 | 392 | 95.37% | 31.8 ms | 339.5 ms | 3293.2 ms |
| All | 26838 | 16238 | 0 | 10600 | 60.50% | 25.6 ms | 235.3 ms | 3293.2 ms |

Reads during the load: 1794 succeeded out of 2936.

Final reads: 50 succeeded out of 51 attempts, over 1.8 s.

## Sessions

10 sessions were sent 949 turns: 614 acknowledged, 335 with an unknown outcome, 0 failed.

The sink recorded 1226 tool calls for 1226 distinct steps.

616 alarms fired, after their due time by p50 25.0 ms and at most 115801.8 ms.

## Errors

| Count | Error |
| ---: | --- |
| 11611 | `action failed: 503 Service Unavailable` |
| 306 | `queue send failed: 503 Service Unavailable` |
| 60 | `timed out after 5000 ms` |
| 28 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 23 | `action failed (core/internal_error): An internal error occurred, metadata=Some(Object {"error": String("actor response aborted")})` |
| 15 | `action failed (depot/unavailable): The storage host could not finish the command (command_capacity); its outcome may be unknown., metadata=Some(Object {"reason": String("command_capacity")})` |
| 6 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 4 | `action failed: 502 Bad Gateway` |
| 3 | `queue send failed (core/internal_error): An internal error occurred, metadata=Some(Object {"error": String("actor response aborted")})` |
| 3 | `queue send failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 2 | `action failed (depot/unavailable): The storage host could not finish the command (open_failed); its outcome may be unknown., metadata=Some(Object {"reason": String("open_failed")})` |
| 2 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=18&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `action failed (core/internal_error): An internal error occurred, metadata=None` |
| 1 | `action failed (depot/request_deadline_exceeded): The command deadline elapsed; its outcome may be unknown., metadata=None` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=11&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=16&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=21&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=32&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=35&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=36&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=42&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=44&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=46&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=5&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client ` |
| 1 | `queue send failed (depot/unavailable): The storage host could not finish the command (command_capacity); its outcome may be unknown., metadata=Some(Object {"reason": String("command_capacity")})` |
| 1 | `queue send failed (depot/unavailable): The storage host could not finish the command (open_failed); its outcome may be unknown., metadata=Some(Object {"reason": String("open_failed")})` |
| 1 | `queue send failed: 502 Bad Gateway` |
