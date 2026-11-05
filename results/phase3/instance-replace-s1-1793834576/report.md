# instance-replace-s1-1793834576

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-replace` |
| Seed | 1 |
| Started | 2026-11-04 23:24:45 UTC |
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
| I1 | Durability | PASS | 40891 | 0 |
| I2 | No phantom writes | PASS | 40915 | 0 |
| I3 | No duplicates | PASS | 40915 | 0 |
| I4 | Real-time visibility | PASS | 4517 | 0 |
| I5 | Recovery | PASS | 58 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 2496 | 0 |
| I8 | Alarms | PASS | 1284 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | replace | engine-1 | 60.0 s | 93.6 s | 33.6 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 58 | 0 | 2.1 s | 8.7 s | 40.1 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6548 | 6548 | 0 | 0 | 100.00% | 7.4 ms | 14.3 ms | 107.6 ms |
| During faults | 128 | 18 | 0 | 110 | 14.06% | 3178.5 ms | 4579.3 ms | 4579.3 ms |
| After faults | 34325 | 34325 | 0 | 0 | 100.00% | 8.1 ms | 31.2 ms | 4755.5 ms |
| All | 41001 | 40891 | 0 | 110 | 99.73% | 8.0 ms | 29.1 ms | 4755.5 ms |

Reads during the load: 4467 succeeded out of 4480.

Final reads: 50 succeeded out of 50 attempts, over 0.5 s.

## Sessions

10 sessions were sent 1325 turns: 1275 acknowledged, 50 with an unknown outcome, 0 failed.

The sink recorded 2496 tool calls for 2496 distinct steps.

1284 alarms fired, after their due time by p50 7.6 ms and at most 34939.4 ms.

## Errors

| Count | Error |
| ---: | --- |
| 115 | `timed out after 5000 ms` |
| 33 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 15 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 3 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 1 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (ups_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("ups_endpoint_unresolved"), "retry_after_ms": Numb` |
| 1 | `action failed (ups/transport): Worker transport failed; the operation may have executed., metadata=None` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=37&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `queue send failed (core/internal_error): An internal error occurred, metadata=Some(Object {"error": String("actor response aborted")})` |
| 1 | `queue send failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 1 | `queue send failed (sqlite/transaction_closed): An internal error occurred, metadata=None` |
