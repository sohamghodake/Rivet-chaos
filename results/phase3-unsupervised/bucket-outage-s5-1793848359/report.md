# bucket-outage-s5-1793848359

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `bucket-outage` |
| Seed | 5 |
| Started | 2026-11-05 03:14:30 UTC |
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
| I4 | Real-time visibility | PASS | 1077 | 0 |
| I5 | Recovery | FAIL | 0 | 110 |
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

- ledger/0 never answered again after disconnect s3 from storage
- ledger/1 never answered again after disconnect s3 from storage
- ledger/10 never answered again after disconnect s3 from storage
- ledger/11 never answered again after disconnect s3 from storage
- ledger/12 never answered again after disconnect s3 from storage
- ledger/13 never answered again after disconnect s3 from storage
- ledger/14 never answered again after disconnect s3 from storage
- ledger/15 never answered again after disconnect s3 from storage
- ledger/16 never answered again after disconnect s3 from storage
- ledger/17 never answered again after disconnect s3 from storage
- ledger/18 never answered again after disconnect s3 from storage
- ledger/19 never answered again after disconnect s3 from storage
- ledger/2 never answered again after disconnect s3 from storage
- ledger/20 never answered again after disconnect s3 from storage
- ledger/21 never answered again after disconnect s3 from storage
- ledger/22 never answered again after disconnect s3 from storage
- ledger/23 never answered again after disconnect s3 from storage
- ledger/24 never answered again after disconnect s3 from storage
- ledger/25 never answered again after disconnect s3 from storage
- ledger/26 never answered again after disconnect s3 from storage
- ledger/27 never answered again after disconnect s3 from storage
- ledger/28 never answered again after disconnect s3 from storage
- ledger/29 never answered again after disconnect s3 from storage
- ledger/3 never answered again after disconnect s3 from storage
- ledger/30 never answered again after disconnect s3 from storage
- ledger/31 never answered again after disconnect s3 from storage
- ledger/32 never answered again after disconnect s3 from storage
- ledger/33 never answered again after disconnect s3 from storage
- ledger/34 never answered again after disconnect s3 from storage
- ledger/35 never answered again after disconnect s3 from storage
- ledger/36 never answered again after disconnect s3 from storage
- ledger/37 never answered again after disconnect s3 from storage
- ledger/38 never answered again after disconnect s3 from storage
- ledger/39 never answered again after disconnect s3 from storage
- ledger/4 never answered again after disconnect s3 from storage
- ledger/40 never answered again after disconnect s3 from storage
- ledger/41 never answered again after disconnect s3 from storage
- ledger/42 never answered again after disconnect s3 from storage
- ledger/43 never answered again after disconnect s3 from storage
- ledger/44 never answered again after disconnect s3 from storage
- ledger/45 never answered again after disconnect s3 from storage
- ledger/46 never answered again after disconnect s3 from storage
- ledger/47 never answered again after disconnect s3 from storage
- ledger/48 never answered again after disconnect s3 from storage
- ledger/49 never answered again after disconnect s3 from storage
- ledger/5 never answered again after disconnect s3 from storage
- ledger/6 never answered again after disconnect s3 from storage
- ledger/7 never answered again after disconnect s3 from storage
- ledger/8 never answered again after disconnect s3 from storage
- ledger/9 never answered again after disconnect s3 from storage
- … and 60 more

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `storage` | s3 | 60.0 s | 181.4 s | 121.4 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 0 | - | - | - | - |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6480 | 6480 | 0 | 0 | 100.00% | 7.1 ms | 15.0 ms | 33.5 ms |
| During faults | 13074 | 2907 | 0 | 10167 | 22.23% | 9.0 ms | 24.0 ms | 116.6 ms |
| After faults | 13520 | 0 | 0 | 13520 | 0.00% | - | - | - |
| All | 33074 | 9387 | 0 | 23687 | 28.38% | 7.6 ms | 19.3 ms | 116.6 ms |

Reads during the load: 1077 succeeded out of 3636.

Final reads: 0 succeeded out of 15013 attempts, over 301.0 s.

## Sessions

10 sessions were sent 993 turns: 294 acknowledged, 699 with an unknown outcome, 0 failed.

The sink recorded 589 tool calls for 589 distinct steps.

290 alarms fired, after their due time by p50 7.4 ms and at most 31.7 ms.

## Errors

| Count | Error |
| ---: | --- |
| 44232 | `action failed: 503 Service Unavailable` |
| 692 | `queue send failed: 503 Service Unavailable` |
| 22 | `timed out after 5000 ms` |
| 6 | `action failed: 502 Bad Gateway` |
| 2 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=49&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=0&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=11&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=29&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=34&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=45&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `queue send failed (depot/unavailable): The storage host could not finish the command (transport); its outcome may be unknown., metadata=Some(Object {"reason": String("transport")})` |
