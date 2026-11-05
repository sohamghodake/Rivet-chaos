# instance-down-long-s4-1793830011

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-down-long` |
| Seed | 4 |
| Started | 2026-11-04 22:08:46 UTC |
| Engine image | `rivetdev/engine:3.0.0@sha256:5ae3c24275eeff87ad1962f4d66fcdb85786fc1397feeeb55afc2ca1f7ccf6f2` |
| rivet-chaos commit | 9f5be2f |
| rivetkit | 3.0.0 |
| rivetkit-client | 3.0.0 |
| CPUs | 16 |
| Docker | 29.0.1 on Docker Desktop, kernel 6.6.87.2-microsoft-standard-WSL2 |
| Memory | 7.4 GiB |

## Invariants

| ID | Invariant | Result | Checked | Violations |
| --- | --- | --- | ---: | ---: |
| I1 | Durability | PASS | 13204 | 0 |
| I2 | No phantom writes | PASS | 13347 | 0 |
| I3 | No duplicates | PASS | 13347 | 0 |
| I4 | Real-time visibility | PASS | 1472 | 0 |
| I5 | Recovery | PASS | 60 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1688 | 0 |
| I8 | Alarms | PASS | 863 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-1 | 60.0 s | 243.6 s | 183.6 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 60 | 46 | -140.1 s | 6.8 s | 190.2 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6517 | 6517 | 0 | 0 | 100.00% | 6.9 ms | 14.9 ms | 70.9 ms |
| During faults | 1641 | 1139 | 0 | 502 | 69.41% | 6.7 ms | 1693.7 ms | 4866.0 ms |
| After faults | 5557 | 5548 | 0 | 9 | 99.84% | 7.6 ms | 17.5 ms | 4526.1 ms |
| All | 13715 | 13204 | 0 | 511 | 96.27% | 7.2 ms | 17.5 ms | 4866.0 ms |

Reads during the load: 1422 succeeded out of 1467.

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Sessions

10 sessions were sent 909 turns: 841 acknowledged, 68 with an unknown outcome, 0 failed.

The sink recorded 1688 tool calls for 1688 distinct steps.

863 alarms fired, after their due time by p50 6.8 ms and at most 34722.8 ms.

## Errors

| Count | Error |
| ---: | --- |
| 581 | `timed out after 5000 ms` |
| 22 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 16 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 3 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/agent-session/queue/message?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=5&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=8&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client ` |
