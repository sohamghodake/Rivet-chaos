# instance-isolation-s1-1793842420

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `instance-isolation` |
| Seed | 1 |
| Started | 2026-11-05 01:35:40 UTC |
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
| I1 | Durability | PASS | 16112 | 0 |
| I2 | No phantom writes | PASS | 16280 | 0 |
| I3 | No duplicates | PASS | 16280 | 0 |
| I4 | Real-time visibility | PASS | 1769 | 0 |
| I5 | Recovery | PASS | 117 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1308 | 0 |
| I8 | Alarms | FAIL | 685 | 6 |

### I8 violations

- alarm 1/56 never fired
- alarm 2/21 never fired
- alarm 2/58 never fired
- alarm 3/65 never fired
- alarm 4/65 never fired
- alarm 6/61 never fired

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `peer` | engine-1 | 60.0 s | 120.4 s | 60.4 s |
| 2 | disconnect from `peer` | engine-3 | 210.0 s | 270.4 s | 60.4 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 58 | 20 | 1.3 s | 30.8 s | 91.1 s |
| 2 | 59 | 8 | 1.7 s | 7.9 s | 68.3 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6558 | 6558 | 0 | 0 | 100.00% | 7.2 ms | 14.8 ms | 108.4 ms |
| During faults | 422 | 80 | 0 | 342 | 18.96% | 7.9 ms | 4857.9 ms | 4857.9 ms |
| After faults | 9546 | 9474 | 0 | 72 | 99.25% | 7.4 ms | 41.5 ms | 4927.5 ms |
| All | 16526 | 16112 | 0 | 414 | 97.49% | 7.3 ms | 30.6 ms | 4927.5 ms |

Reads during the load: 1719 succeeded out of 1754.

Final reads: 50 succeeded out of 50 attempts, over 4.8 s.

## Sessions

10 sessions were sent 783 turns: 653 acknowledged, 130 with an unknown outcome, 0 failed.

The sink recorded 1308 tool calls for 1308 distinct steps.

679 alarms fired, after their due time by p50 6.9 ms and at most 121.3 ms.

## Errors

| Count | Error |
| ---: | --- |
| 543 | `timed out after 5000 ms` |
| 12 | `action failed (depot/overloaded): The storage host refused the command (endpoint_unresolved); retry after 100 ms., metadata=None` |
| 7 | `queue send failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_m` |
| 6 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 5 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 2 | `queue send failed (sqlite/transaction_closed): An internal error occurred, metadata=None` |
| 1 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 1 | `action failed (sqlite/closed): An internal error occurred, metadata=None` |
| 1 | `queue send failed (depot/overloaded): The storage host refused the command (endpoint_unresolved); retry after 100 ms., metadata=None` |
| 1 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
