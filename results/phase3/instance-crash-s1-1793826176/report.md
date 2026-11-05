# instance-crash-s1-1793826176

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-crash` |
| Seed | 1 |
| Started | 2026-11-04 21:04:54 UTC |
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
| I1 | Durability | PASS | 24365 | 0 |
| I2 | No phantom writes | PASS | 24427 | 0 |
| I3 | No duplicates | PASS | 24427 | 0 |
| I4 | Real-time visibility | PASS | 2690 | 0 |
| I5 | Recovery | PASS | 92 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1628 | 0 |
| I8 | Alarms | PASS | 843 | 0 |

I8: 1 alarms fired on time by the wall clock but early by the monotonic clock, as a forward wall-clock adjustment causes.

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-1 | 60.0 s | 94.2 s | 34.2 s |
| 2 | kill | engine-3 | 180.0 s | 213.3 s | 33.2 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 52 | 10 | 1.0 s | 6.2 s | 40.1 s |
| 2 | 40 | 10 | 1.9 s | 2.8 s | 36.0 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6462 | 6462 | 0 | 0 | 100.00% | 8.3 ms | 30.7 ms | 207.2 ms |
| During faults | 334 | 124 | 0 | 210 | 37.13% | 10.2 ms | 4788.2 ms | 4931.6 ms |
| After faults | 17779 | 17779 | 0 | 0 | 100.00% | 8.3 ms | 26.4 ms | 4870.1 ms |
| All | 24575 | 24365 | 0 | 210 | 99.15% | 8.3 ms | 31.9 ms | 4931.6 ms |

Reads during the load: 2640 succeeded out of 2666.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 894 turns: 829 acknowledged, 65 with an unknown outcome, 0 failed.

The sink recorded 1628 tool calls for 1628 distinct steps.

843 alarms fired, after their due time by p50 7.9 ms and at most 29491.4 ms.

## Errors

| Count | Error |
| ---: | --- |
| 217 | `timed out after 5000 ms` |
| 51 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 14 | `queue send failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_m` |
| 8 | `action failed (guard/service_unavailable): Service unavailable (ups_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("ups_endpoint_unresolved"), "retry_after_ms": Numb` |
| 2 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 2 | `queue send failed (guard/service_unavailable): Service unavailable (ups_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("ups_endpoint_unresolved"), "retry_after_ms": ` |
| 2 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=14&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=28&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=4&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=41&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=48&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
