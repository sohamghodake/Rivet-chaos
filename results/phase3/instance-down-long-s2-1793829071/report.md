# instance-down-long-s2-1793829071

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-down-long` |
| Seed | 2 |
| Started | 2026-11-04 21:52:28 UTC |
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
| I1 | Durability | PASS | 28059 | 0 |
| I2 | No phantom writes | PASS | 28060 | 0 |
| I3 | No duplicates | PASS | 28060 | 0 |
| I4 | Real-time visibility | PASS | 3169 | 0 |
| I5 | Recovery | PASS | 20 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1943 | 0 |
| I8 | Alarms | PASS | 985 | 0 |

I8: 1 alarms fired on time by the wall clock but early by the monotonic clock, as a forward wall-clock adjustment causes.

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-3 | 60.0 s | 245.0 s | 185.0 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 20 | 19 | -153.3 s | 0.8 s | 32.8 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6337 | 6337 | 0 | 0 | 100.00% | 8.8 ms | 33.3 ms | 141.8 ms |
| During faults | 16054 | 15966 | 0 | 88 | 99.45% | 9.4 ms | 156.9 ms | 4718.6 ms |
| After faults | 5756 | 5756 | 0 | 0 | 100.00% | 9.1 ms | 46.3 ms | 155.8 ms |
| All | 28147 | 28059 | 0 | 88 | 99.69% | 9.1 ms | 104.4 ms | 4718.6 ms |

Reads during the load: 3119 succeeded out of 3130.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 994 turns: 984 acknowledged, 10 with an unknown outcome, 0 failed.

The sink recorded 1943 tool calls for 1943 distinct steps.

985 alarms fired, after their due time by p50 8.7 ms and at most 348.0 ms.

## Errors

| Count | Error |
| ---: | --- |
| 67 | `timed out after 5000 ms` |
| 30 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 5 | `queue send failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_m` |
| 3 | `action failed (depot/overloaded): The storage host refused the command (endpoint_unresolved); retry after 100 ms., metadata=None` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=20&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=28&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/read?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=13&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client e` |
