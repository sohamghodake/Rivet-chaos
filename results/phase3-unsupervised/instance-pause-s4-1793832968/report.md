# instance-pause-s4-1793832968

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `instance-pause` |
| Seed | 4 |
| Started | 2026-11-04 22:58:03 UTC |
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
| I1 | Durability | UNKNOWN | 0 | 0 |
| I2 | No phantom writes | UNKNOWN | 0 | 0 |
| I3 | No duplicates | UNKNOWN | 0 | 0 |
| I4 | Real-time visibility | PASS | 759 | 0 |
| I5 | Recovery | FAIL | 13 | 141 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | UNKNOWN | 0 | 0 |
| I8 | Alarms | UNKNOWN | 0 | 0 |

50 ledgers had no successful read after their last append, so they were not checked: ledger/0, ledger/1, ledger/10, ledger/11, ledger/12, ledger/13, ledger/14, ledger/15, ledger/16, ledger/17, ledger/18, ledger/19, ledger/2, ledger/20, ledger/21, ledger/22, ledger/23, ledger/24, ledger/25, ledger/26, ledger/27, ledger/28, ledger/29, ledger/3, ledger/30, ledger/31, ledger/32, ledger/33, ledger/34, ledger/35, ledger/36, ledger/37, ledger/38, ledger/39, ledger/4, ledger/40, ledger/41, ledger/42, ledger/43, ledger/44, ledger/45, ledger/46, ledger/47, ledger/48, ledger/49, ledger/5, ledger/6, ledger/7, ledger/8, ledger/9

I7: session/0 was never read after its last turn was sent, so its turns weren't checked.

I7: session/1 was never read after its last turn was sent, so its turns weren't checked.

I7: session/2 was never read after its last turn was sent, so its turns weren't checked.

I7: session/3 was never read after its last turn was sent, so its turns weren't checked.

I7: session/4 was never read after its last turn was sent, so its turns weren't checked.

I7: session/5 was never read after its last turn was sent, so its turns weren't checked.

I7: session/6 was never read after its last turn was sent, so its turns weren't checked.

I7: session/7 was never read after its last turn was sent, so its turns weren't checked.

I7: session/8 was never read after its last turn was sent, so its turns weren't checked.

I7: session/9 was never read after its last turn was sent, so its turns weren't checked.

### I5 violations

- ledger/0 never answered again after pause engine-1
- ledger/10 never answered again after pause engine-1
- ledger/11 never answered again after pause engine-1
- ledger/12 never answered again after pause engine-1
- ledger/16 never answered again after pause engine-1
- ledger/17 never answered again after pause engine-1
- ledger/19 never answered again after pause engine-1
- ledger/2 never answered again after pause engine-1
- ledger/21 never answered again after pause engine-1
- ledger/26 never answered again after pause engine-1
- ledger/28 never answered again after pause engine-1
- ledger/29 never answered again after pause engine-1
- ledger/3 never answered again after pause engine-1
- ledger/30 never answered again after pause engine-1
- ledger/32 never answered again after pause engine-1
- ledger/34 never answered again after pause engine-1
- ledger/38 never answered again after pause engine-1
- ledger/4 never answered again after pause engine-1
- ledger/40 never answered again after pause engine-1
- ledger/41 never answered again after pause engine-1
- ledger/43 never answered again after pause engine-1
- ledger/47 never answered again after pause engine-1
- ledger/49 never answered again after pause engine-1
- ledger/6 never answered again after pause engine-1
- ledger/7 never answered again after pause engine-1
- ledger/9 never answered again after pause engine-1
- session/0 never answered again after pause engine-1
- session/1 never answered again after pause engine-1
- session/2 never answered again after pause engine-1
- session/3 never answered again after pause engine-1
- session/6 never answered again after pause engine-1
- session/8 never answered again after pause engine-1
- ledger/0 never answered again after pause engine-3
- ledger/10 never answered again after pause engine-3
- ledger/11 never answered again after pause engine-3
- ledger/12 never answered again after pause engine-3
- ledger/13 never answered again after pause engine-3
- ledger/14 never answered again after pause engine-3
- ledger/15 never answered again after pause engine-3
- ledger/16 never answered again after pause engine-3
- ledger/17 never answered again after pause engine-3
- ledger/18 never answered again after pause engine-3
- ledger/19 never answered again after pause engine-3
- ledger/2 never answered again after pause engine-3
- ledger/20 never answered again after pause engine-3
- ledger/21 never answered again after pause engine-3
- ledger/22 never answered again after pause engine-3
- ledger/23 never answered again after pause engine-3
- ledger/24 never answered again after pause engine-3
- ledger/25 never answered again after pause engine-3
- … and 91 more

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | pause | engine-1 | 60.0 s | 105.1 s | 45.1 s |
| 2 | pause | engine-3 | 180.0 s | 225.1 s | 45.1 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 13 | 9 | -9.6 s | 15.8 s | 60.6 s |
| 2 | 0 | - | - | - | - |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6537 | 6537 | 0 | 0 | 100.00% | 6.7 ms | 11.7 ms | 58.6 ms |
| During faults | 307 | 50 | 0 | 257 | 16.29% | 7.4 ms | 4032.5 ms | 4032.5 ms |
| After faults | 614 | 185 | 0 | 429 | 30.13% | 7.1 ms | 40.3 ms | 43.6 ms |
| All | 7458 | 6772 | 0 | 686 | 90.80% | 6.7 ms | 13.1 ms | 4032.5 ms |

Reads during the load: 759 succeeded out of 828.

Final reads: 0 succeeded out of 2748 attempts, over 305.2 s.

## Sessions

10 sessions were sent 581 turns: 328 acknowledged, 253 with an unknown outcome, 0 failed.

The sink recorded 645 tool calls for 645 distinct steps.

322 alarms fired, after their due time by p50 6.6 ms and at most 12.9 ms.

## Errors

| Count | Error |
| ---: | --- |
| 3867 | `timed out after 5000 ms` |
| 262 | `action failed (guard/service_unavailable): Service unavailable (ups_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("ups_endpoint_unresolved"), "retry_after_ms": Numb` |
| 112 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 13 | `action failed (depot/request_deadline_exceeded): The command deadline elapsed; its outcome may be unknown., metadata=None` |
| 5 | `action failed (ups/transport): Worker transport failed; the operation may have executed., metadata=None` |
| 2 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 123 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(123)})` |
| 2 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 141 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(141)})` |
| 2 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 150 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(150)})` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 112 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(112)})` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 134 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(134)})` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 146 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(146)})` |
