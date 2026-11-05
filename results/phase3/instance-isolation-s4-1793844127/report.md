# instance-isolation-s4-1793844127

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `instance-isolation` |
| Seed | 4 |
| Started | 2026-11-05 02:04:06 UTC |
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
| I1 | Durability | PASS | 16493 | 0 |
| I2 | No phantom writes | PASS | 16631 | 0 |
| I3 | No duplicates | PASS | 16631 | 0 |
| I4 | Real-time visibility | PASS | 1827 | 0 |
| I5 | Recovery | PASS | 114 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1200 | 0 |
| I8 | Alarms | FAIL | 622 | 7 |

### I8 violations

- alarm 2/21 never fired
- alarm 2/58 never fired
- alarm 4/21 never fired
- alarm 6/21 never fired
- alarm 7/56 never fired
- alarm 8/21 never fired
- alarm 9/22 never fired

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `peer` | engine-1 | 60.0 s | 120.4 s | 60.4 s |
| 2 | disconnect from `peer` | engine-3 | 210.0 s | 270.4 s | 60.4 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 58 | 0 | 2.1 s | 5.7 s | 65.5 s |
| 2 | 56 | 6 | 0.7 s | 60.7 s | 121.0 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6509 | 6509 | 0 | 0 | 100.00% | 7.1 ms | 14.3 ms | 60.8 ms |
| During faults | 394 | 45 | 0 | 349 | 11.42% | 8.8 ms | 4886.5 ms | 4886.5 ms |
| After faults | 9999 | 9939 | 0 | 60 | 99.40% | 7.2 ms | 30.2 ms | 4939.8 ms |
| All | 16902 | 16493 | 0 | 409 | 97.58% | 7.2 ms | 20.1 ms | 4939.8 ms |

Reads during the load: 1777 succeeded out of 1812.

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Sessions

10 sessions were sent 751 turns: 602 acknowledged, 149 with an unknown outcome, 0 failed.

The sink recorded 1200 tool calls for 1200 distinct steps.

615 alarms fired, after their due time by p50 6.8 ms and at most 2628.4 ms.

## Errors

| Count | Error |
| ---: | --- |
| 558 | `timed out after 5000 ms` |
| 16 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 5 | `action failed (sqlite/closed): An internal error occurred, metadata=None` |
| 4 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 4 | `queue send failed (sqlite/transaction_closed): An internal error occurred, metadata=None` |
| 3 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 2 | `queue send failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_m` |
| 1 | `action failed (depot/request_deadline_exceeded): The command deadline elapsed; its outcome may be unknown., metadata=None` |
