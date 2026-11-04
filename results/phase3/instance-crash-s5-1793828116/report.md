# instance-crash-s5-1793828116

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `instance-crash` |
| Seed | 5 |
| Started | 2026-11-04 21:36:29 UTC |
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
| I1 | Durability | PASS | 22675 | 0 |
| I2 | No phantom writes | PASS | 22752 | 0 |
| I3 | No duplicates | PASS | 22752 | 0 |
| I4 | Real-time visibility | PASS | 2548 | 0 |
| I5 | Recovery | PASS | 81 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1618 | 0 |
| I8 | Alarms | FAIL | 827 | 1 |

I7: 2 tool calls ran more than once, each with a fault during its step.

### I8 violations

- alarm 8/57 was booked 3 times with no fault during its booking step

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-3 | 60.0 s | 94.5 s | 34.5 s |
| 2 | kill | engine-2 | 180.0 s | 217.1 s | 37.1 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 41 | 15 | 2.9 s | 61.0 s | 94.5 s |
| 2 | 40 | 15 | 2.3 s | 13.0 s | 46.3 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6339 | 6339 | 0 | 0 | 100.00% | 8.1 ms | 55.2 ms | 131.2 ms |
| During faults | 470 | 248 | 0 | 222 | 52.77% | 20.7 ms | 4788.2 ms | 4919.3 ms |
| After faults | 16088 | 16088 | 0 | 0 | 100.00% | 9.4 ms | 268.0 ms | 3698.7 ms |
| All | 22897 | 22675 | 0 | 222 | 99.03% | 9.0 ms | 244.1 ms | 4919.3 ms |

Reads during the load: 2498 succeeded out of 2530.

Final reads: 50 succeeded out of 50 attempts, over 0.4 s.

## Sessions

10 sessions were sent 865 turns: 784 acknowledged, 81 with an unknown outcome, 0 failed.

The sink recorded 1626 tool calls for 1624 distinct steps.

831 alarms fired, after their due time by p50 8.8 ms and at most 180336.4 ms.

## Errors

| Count | Error |
| ---: | --- |
| 244 | `timed out after 5000 ms` |
| 40 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 19 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 8 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 4 | `action failed (sqlite/closed): An internal error occurred, metadata=None` |
| 4 | `queue send failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_m` |
| 3 | `action failed (guard/service_unavailable): Service unavailable (ups_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("ups_endpoint_unresolved"), "retry_after_ms": Numb` |
| 3 | `action failed (ups/transport): Worker transport failed; the operation may have executed., metadata=None` |
| 2 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=25&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 2 | `queue send failed (sqlite/transaction_closed): An internal error occurred, metadata=None` |
| 1 | `action failed (depot/overloaded): The storage host refused the command (endpoint_unresolved); retry after 100 ms., metadata=None` |
| 1 | `error sending request for url (http://localhost:6420/gateway/agent-session/queue/message?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=8&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/agent-session/queue/message?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=9&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=11&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=33&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=35&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=39&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=42&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `queue send failed (ups/transport): Worker transport failed; the operation may have executed., metadata=None` |
