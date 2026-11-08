# bucket-outage-s1-1794157585

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `bucket-outage` |
| Seed | 1 |
| Started | 2026-11-08 17:07:45 UTC |
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
| I1 | Durability | PASS | 15748 | 0 |
| I2 | No phantom writes | PASS | 15751 | 0 |
| I3 | No duplicates | PASS | 15751 | 0 |
| I4 | Real-time visibility | PASS | 1781 | 0 |
| I5 | Recovery | PASS | 60 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1183 | 0 |
| I8 | Alarms | FAIL | 599 | 2 |

### I8 violations

- alarm 2/68 was booked 3 times, due at 207.1 s, 267.5 s, 327.7 s, with no fault during its booking step
- alarm 6/64 was booked 3 times, due at 203.2 s, 264.8 s, 325.0 s, with no fault during its booking step

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `storage` | s3 | 60.0 s | 183.3 s | 123.3 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 60 | 0 | 10.5 s | 13.4 s | 110.0 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 5518 | 5518 | 0 | 0 | 100.00% | 26.0 ms | 127.3 ms | 227.2 ms |
| During faults | 12169 | 1952 | 0 | 10217 | 16.04% | 43.5 ms | 483.3 ms | 1131.5 ms |
| After faults | 8651 | 8278 | 0 | 373 | 95.69% | 37.6 ms | 416.8 ms | 2555.9 ms |
| All | 26338 | 15748 | 0 | 10590 | 59.79% | 32.9 ms | 379.6 ms | 2555.9 ms |

Reads during the load: 1731 succeeded out of 2875.

Final reads: 50 succeeded out of 53 attempts, over 1.6 s.

## Sessions

10 sessions were sent 931 turns: 576 acknowledged, 355 with an unknown outcome, 0 failed.

The sink recorded 1183 tool calls for 1183 distinct steps.

603 alarms fired, after their due time by p50 31.8 ms and at most 120875.6 ms.

## Errors

| Count | Error |
| ---: | --- |
| 11586 | `action failed: 503 Service Unavailable` |
| 317 | `queue send failed: 503 Service Unavailable` |
| 76 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 42 | `timed out after 5000 ms` |
| 16 | `action failed (depot/unavailable): The storage host could not finish the command (command_capacity); its outcome may be unknown., metadata=Some(Object {"reason": String("command_capacity")})` |
| 15 | `action failed (core/internal_error): An internal error occurred, metadata=Some(Object {"error": String("actor response aborted")})` |
| 9 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 9 | `queue send failed (depot/unavailable): The storage host could not finish the command (command_capacity); its outcome may be unknown., metadata=Some(Object {"reason": String("command_capacity")})` |
| 3 | `action failed (core/internal_error): An internal error occurred, metadata=None` |
| 2 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=6&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client ` |
| 2 | `queue send failed (core/internal_error): An internal error occurred, metadata=Some(Object {"error": String("actor response aborted")})` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=1&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=17&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=18&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=31&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=36&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=38&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=41&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=47&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=48&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=7&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=8&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/read?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=12&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client e` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/read?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=37&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client e` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/read?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=45&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client e` |
| 1 | `queue send failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
