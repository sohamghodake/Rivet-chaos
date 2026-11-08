# bucket-outage-s2-1794155689

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `bucket-outage` |
| Seed | 2 |
| Started | 2026-11-08 16:36:47 UTC |
| Engine image | `rivetdev/engine:3.0.0@sha256:5ae3c24275eeff87ad1962f4d66fcdb85786fc1397feeeb55afc2ca1f7ccf6f2` |
| rivet-chaos commit | 6388a11 |
| rivetkit | 3.0.0 |
| rivetkit-client | 3.0.0 |
| CPUs | 16 |
| Docker | 29.0.1 on Docker Desktop, kernel 6.6.87.2-microsoft-standard-WSL2 |
| Memory | 7.4 GiB |

## Invariants

| ID | Invariant | Result | Checked | Violations |
| --- | --- | --- | ---: | ---: |
| I1 | Durability | PASS | 21493 | 0 |
| I2 | No phantom writes | PASS | 21493 | 0 |
| I3 | No duplicates | PASS | 21493 | 0 |
| I4 | Real-time visibility | PASS | 2517 | 0 |
| I5 | Recovery | PASS | 60 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1368 | 0 |
| I8 | Alarms | FAIL | 681 | 1 |

### I8 violations

- alarm 7/30 was booked 2 times, due at 87.3 s, 186.8 s, with no fault during its booking step

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `storage` | s3 | 60.0 s | 181.9 s | 121.9 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 60 | 0 | 4.0 s | 6.9 s | 101.2 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6422 | 6422 | 0 | 0 | 100.00% | 7.4 ms | 23.0 ms | 55.5 ms |
| During faults | 13082 | 2812 | 0 | 10270 | 21.50% | 9.1 ms | 22.9 ms | 40.8 ms |
| After faults | 12411 | 12259 | 0 | 152 | 98.78% | 7.4 ms | 29.8 ms | 2566.1 ms |
| All | 31915 | 21493 | 0 | 10422 | 67.34% | 7.6 ms | 24.6 ms | 2566.1 ms |

Reads during the load: 2467 succeeded out of 3583.

Final reads: 50 succeeded out of 50 attempts, over 0.4 s.

## Sessions

10 sessions were sent 995 turns: 681 acknowledged, 314 with an unknown outcome, 0 failed.

The sink recorded 1368 tool calls for 1368 distinct steps.

682 alarms fired, after their due time by p50 7.5 ms and at most 101789.1 ms.

## Errors

| Count | Error |
| ---: | --- |
| 11477 | `action failed: 503 Service Unavailable` |
| 305 | `queue send failed: 503 Service Unavailable` |
| 18 | `timed out after 5000 ms` |
| 14 | `action failed: 502 Bad Gateway` |
| 13 | `action failed (depot/unavailable): The storage host could not finish the command (command_capacity); its outcome may be unknown., metadata=Some(Object {"reason": String("command_capacity")})` |
| 10 | `action failed (core/internal_error): An internal error occurred, metadata=Some(Object {"error": String("actor response aborted")})` |
| 4 | `queue send failed (core/internal_error): An internal error occurred, metadata=Some(Object {"error": String("actor response aborted")})` |
| 3 | `queue send failed (depot/unavailable): The storage host could not finish the command (command_capacity); its outcome may be unknown., metadata=Some(Object {"reason": String("command_capacity")})` |
| 1 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=13&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=15&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=18&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=24&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=36&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=48&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/read?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=45&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client e` |
