# instance-replace-s3-1793835797

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-replace` |
| Seed | 3 |
| Started | 2026-11-04 23:45:15 UTC |
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
| I1 | Durability | PASS | 40999 | 0 |
| I2 | No phantom writes | PASS | 41015 | 0 |
| I3 | No duplicates | PASS | 41015 | 0 |
| I4 | Real-time visibility | PASS | 4644 | 0 |
| I5 | Recovery | PASS | 57 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 2573 | 0 |
| I8 | Alarms | PASS | 1300 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | replace | engine-1 | 60.0 s | 94.4 s | 34.3 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 57 | 0 | 1.7 s | 6.5 s | 38.9 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6503 | 6503 | 0 | 0 | 100.00% | 7.1 ms | 13.1 ms | 46.4 ms |
| During faults | 131 | 16 | 0 | 115 | 12.21% | 1394.7 ms | 4849.7 ms | 4849.7 ms |
| After faults | 34480 | 34480 | 0 | 0 | 100.00% | 7.7 ms | 22.1 ms | 4468.7 ms |
| All | 41114 | 40999 | 0 | 115 | 99.72% | 7.6 ms | 21.5 ms | 4849.7 ms |

Reads during the load: 4594 succeeded out of 4610.

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Sessions

10 sessions were sent 1337 turns: 1287 acknowledged, 50 with an unknown outcome, 0 failed.

The sink recorded 2573 tool calls for 2573 distinct steps.

1300 alarms fired, after their due time by p50 7.3 ms and at most 38582.2 ms.

## Errors

| Count | Error |
| ---: | --- |
| 118 | `timed out after 5000 ms` |
| 37 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 19 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 3 | `action failed (guard/service_unavailable): Service unavailable (ups_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("ups_endpoint_unresolved"), "retry_after_ms": Numb` |
| 1 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 1 | `action failed: 502 Bad Gateway` |
| 1 | `error sending request for url (http://localhost:6420/gateway/agent-session/queue/message?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=5&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=16&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
