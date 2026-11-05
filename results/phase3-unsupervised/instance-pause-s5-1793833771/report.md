# instance-pause-s5-1793833771

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `instance-pause` |
| Seed | 5 |
| Started | 2026-11-04 23:11:27 UTC |
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
| I4 | Real-time visibility | PASS | 1705 | 0 |
| I5 | Recovery | FAIL | 27 | 110 |
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

- ledger/0 never answered again after pause engine-2
- ledger/1 never answered again after pause engine-2
- ledger/10 never answered again after pause engine-2
- ledger/11 never answered again after pause engine-2
- ledger/12 never answered again after pause engine-2
- ledger/13 never answered again after pause engine-2
- ledger/14 never answered again after pause engine-2
- ledger/15 never answered again after pause engine-2
- ledger/16 never answered again after pause engine-2
- ledger/17 never answered again after pause engine-2
- ledger/18 never answered again after pause engine-2
- ledger/19 never answered again after pause engine-2
- ledger/2 never answered again after pause engine-2
- ledger/20 never answered again after pause engine-2
- ledger/21 never answered again after pause engine-2
- ledger/22 never answered again after pause engine-2
- ledger/23 never answered again after pause engine-2
- ledger/24 never answered again after pause engine-2
- ledger/25 never answered again after pause engine-2
- ledger/26 never answered again after pause engine-2
- ledger/27 never answered again after pause engine-2
- ledger/28 never answered again after pause engine-2
- ledger/29 never answered again after pause engine-2
- ledger/3 never answered again after pause engine-2
- ledger/30 never answered again after pause engine-2
- ledger/31 never answered again after pause engine-2
- ledger/32 never answered again after pause engine-2
- ledger/33 never answered again after pause engine-2
- ledger/34 never answered again after pause engine-2
- ledger/35 never answered again after pause engine-2
- ledger/36 never answered again after pause engine-2
- ledger/37 never answered again after pause engine-2
- ledger/38 never answered again after pause engine-2
- ledger/39 never answered again after pause engine-2
- ledger/4 never answered again after pause engine-2
- ledger/40 never answered again after pause engine-2
- ledger/41 never answered again after pause engine-2
- ledger/42 never answered again after pause engine-2
- ledger/43 never answered again after pause engine-2
- ledger/44 never answered again after pause engine-2
- ledger/45 never answered again after pause engine-2
- ledger/46 never answered again after pause engine-2
- ledger/47 never answered again after pause engine-2
- ledger/48 never answered again after pause engine-2
- ledger/49 never answered again after pause engine-2
- ledger/5 never answered again after pause engine-2
- ledger/6 never answered again after pause engine-2
- ledger/7 never answered again after pause engine-2
- ledger/8 never answered again after pause engine-2
- ledger/9 never answered again after pause engine-2
- … and 60 more

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | pause | engine-3 | 60.0 s | 105.1 s | 45.1 s |
| 2 | pause | engine-2 | 180.0 s | 225.1 s | 45.1 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 27 | 21 | -5.1 s | 0.1 s | 45.1 s |
| 2 | 0 | - | - | - | - |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6473 | 6473 | 0 | 0 | 100.00% | 7.3 ms | 15.1 ms | 111.4 ms |
| During faults | 583 | 342 | 0 | 241 | 58.66% | 7.3 ms | 3926.0 ms | 4968.4 ms |
| After faults | 8293 | 8071 | 0 | 222 | 97.32% | 7.2 ms | 15.1 ms | 107.5 ms |
| All | 15349 | 14886 | 0 | 463 | 96.98% | 7.3 ms | 16.7 ms | 4968.4 ms |

Reads during the load: 1705 succeeded out of 1747.

Final reads: 0 succeeded out of 5952 attempts, over 305.2 s.

## Sessions

10 sessions were sent 745 turns: 586 acknowledged, 159 with an unknown outcome, 0 failed.

The sink recorded 1138 tool calls for 1138 distinct steps.

585 alarms fired, after their due time by p50 7.1 ms and at most 48.5 ms.

## Errors

| Count | Error |
| ---: | --- |
| 3901 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 2531 | `timed out after 5000 ms` |
| 306 | `action failed (guard/service_unavailable): Service unavailable (ups_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("ups_endpoint_unresolved"), "retry_after_ms": Numb` |
| 79 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 119 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(119)})` |
| 70 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 113 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(113)})` |
| 68 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 110 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(110)})` |
| 68 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 145 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(145)})` |
| 52 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 137 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(137)})` |
| 49 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 112 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(112)})` |
| 46 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 134 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(134)})` |
| 38 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 128 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(128)})` |
| 37 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 114 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(114)})` |
| 36 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 111 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(111)})` |
| 36 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 116 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(116)})` |
| 33 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 122 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(122)})` |
| 31 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 124 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(124)})` |
| 31 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 129 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(129)})` |
| 31 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 133 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(133)})` |
| 30 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 106 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(106)})` |
| 27 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 127 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(127)})` |
| 19 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 108 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(108)})` |
| 18 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 101 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(101)})` |
| 18 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 131 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(131)})` |
| 18 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 136 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(136)})` |
| 18 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 141 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(141)})` |
| 17 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 103 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(103)})` |
| 17 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 104 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(104)})` |
| 17 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 118 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(118)})` |
| 17 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 130 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(130)})` |
| 17 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 148 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(148)})` |
| 17 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 150 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(150)})` |
| 16 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 135 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(135)})` |
| 16 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 139 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(139)})` |
| 16 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 144 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(144)})` |
| 14 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 117 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(117)})` |
| 13 | `action failed (depot/request_deadline_exceeded): The command deadline elapsed; its outcome may be unknown., metadata=None` |
| 13 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 147 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(147)})` |
| 10 | `action failed (sqlite/closed): An internal error occurred, metadata=None` |
| 4 | `queue send failed (sqlite/transaction_closed): An internal error occurred, metadata=None` |
| 2 | `action failed (ups/transport): Worker transport failed; the operation may have executed., metadata=None` |
| 2 | `queue send failed (ups/transport): Worker transport failed; the operation may have executed., metadata=None` |
| 1 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 102 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(102)})` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 126 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(126)})` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 132 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(132)})` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 142 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(142)})` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 143 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(143)})` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 146 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(146)})` |
| 1 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
