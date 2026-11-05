# instance-isolation-s3-1793843421

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `instance-isolation` |
| Seed | 3 |
| Started | 2026-11-05 01:51:37 UTC |
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
| I1 | Durability | PASS | 16629 | 0 |
| I2 | No phantom writes | PASS | 16780 | 0 |
| I3 | No duplicates | PASS | 16780 | 0 |
| I4 | Real-time visibility | PASS | 1870 | 0 |
| I5 | Recovery | FAIL | 113 | 1 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1279 | 0 |
| I8 | Alarms | FAIL | 662 | 5 |

### I5 violations

- session/5 answered again 302.5 s after disconnect engine-2 from peer healed, past the 300.0 s deadline

### I8 violations

- alarm 4/59 never fired
- alarm 5/64 never fired
- alarm 5/65 never fired
- alarm 6/65 never fired
- alarm 7/62 never fired

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `peer` | engine-1 | 60.0 s | 120.4 s | 60.4 s |
| 2 | disconnect from `peer` | engine-2 | 210.0 s | 270.4 s | 60.4 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 56 | 32 | -2.5 s | 2.4 s | 62.0 s |
| 2 | 57 | 0 | 2.0 s | 302.5 s | 362.2 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6504 | 6504 | 0 | 0 | 100.00% | 7.1 ms | 13.5 ms | 25.0 ms |
| During faults | 460 | 132 | 0 | 328 | 28.70% | 8.8 ms | 4612.1 ms | 4726.8 ms |
| After faults | 10058 | 9993 | 0 | 65 | 99.35% | 7.3 ms | 17.7 ms | 4964.4 ms |
| All | 17022 | 16629 | 0 | 393 | 97.69% | 7.2 ms | 16.7 ms | 4964.4 ms |

Reads during the load: 1820 succeeded out of 1861.

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Sessions

10 sessions were sent 768 turns: 629 acknowledged, 139 with an unknown outcome, 0 failed.

The sink recorded 1279 tool calls for 1279 distinct steps.

657 alarms fired, after their due time by p50 7.0 ms and at most 47.2 ms.

## Errors

| Count | Error |
| ---: | --- |
| 592 | `timed out after 5000 ms` |
| 7 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 3 | `action failed (depot/overloaded): The storage host refused the command (endpoint_unresolved); retry after 100 ms., metadata=None` |
| 3 | `action failed (sqlite/closed): An internal error occurred, metadata=None` |
| 2 | `action failed (depot/request_deadline_exceeded): The command deadline elapsed; its outcome may be unknown., metadata=None` |
| 2 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 2 | `queue send failed (sqlite/transaction_closed): An internal error occurred, metadata=None` |
| 1 | `queue send failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 1 | `queue send failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_m` |
