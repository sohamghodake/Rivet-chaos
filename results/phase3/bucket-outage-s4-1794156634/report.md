# bucket-outage-s4-1794156634

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `bucket-outage` |
| Seed | 4 |
| Started | 2026-11-08 16:51:56 UTC |
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
| I1 | Durability | PASS | 17648 | 0 |
| I2 | No phantom writes | PASS | 17657 | 0 |
| I3 | No duplicates | PASS | 17657 | 0 |
| I4 | Real-time visibility | PASS | 1960 | 0 |
| I5 | Recovery | PASS | 60 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1278 | 0 |
| I8 | Alarms | PASS | 637 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `storage` | s3 | 60.0 s | 183.4 s | 123.4 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 60 | 0 | 15.4 s | 21.5 s | 115.4 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 5512 | 5512 | 0 | 0 | 100.00% | 25.8 ms | 121.4 ms | 333.6 ms |
| During faults | 12473 | 2239 | 0 | 10234 | 17.95% | 34.8 ms | 207.5 ms | 351.2 ms |
| After faults | 10101 | 9897 | 0 | 204 | 97.98% | 18.4 ms | 130.4 ms | 4939.8 ms |
| All | 28086 | 17648 | 0 | 10438 | 62.84% | 22.5 ms | 156.9 ms | 4939.8 ms |

Reads during the load: 1910 succeeded out of 2996.

Final reads: 50 succeeded out of 50 attempts, over 0.5 s.

## Sessions

10 sessions were sent 968 turns: 635 acknowledged, 333 with an unknown outcome, 0 failed.

The sink recorded 1278 tool calls for 1278 distinct steps.

637 alarms fired, after their due time by p50 22.1 ms and at most 110376.8 ms.

## Errors

| Count | Error |
| ---: | --- |
| 11433 | `action failed: 503 Service Unavailable` |
| 311 | `queue send failed: 503 Service Unavailable` |
| 45 | `timed out after 5000 ms` |
| 14 | `action failed (core/internal_error): An internal error occurred, metadata=Some(Object {"error": String("actor response aborted")})` |
| 13 | `action failed (depot/unavailable): The storage host could not finish the command (command_capacity); its outcome may be unknown., metadata=Some(Object {"reason": String("command_capacity")})` |
| 9 | `queue send failed (depot/unavailable): The storage host could not finish the command (command_capacity); its outcome may be unknown., metadata=Some(Object {"reason": String("command_capacity")})` |
| 5 | `queue send failed (core/internal_error): An internal error occurred, metadata=Some(Object {"error": String("actor response aborted")})` |
| 4 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 3 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 3 | `action failed: 502 Bad Gateway` |
| 2 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 1 | `action failed (depot/request_deadline_exceeded): The command deadline elapsed; its outcome may be unknown., metadata=None` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=1&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=11&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=24&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=29&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=32&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=34&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=4&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=42&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=8&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/read?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=17&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client e` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/read?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=28&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client e` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/read?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=34&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client e` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/read?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=4&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client er` |
