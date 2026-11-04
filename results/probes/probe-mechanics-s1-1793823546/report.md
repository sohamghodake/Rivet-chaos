# probe-mechanics-s1-1793823546

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `probe-mechanics` |
| Seed | 1 |
| Started | 2026-11-04 20:20:17 UTC |
| Engine image | `rivetdev/engine:3.0.0@sha256:5ae3c24275eeff87ad1962f4d66fcdb85786fc1397feeeb55afc2ca1f7ccf6f2` |
| rivet-chaos commit | 47a596a |
| rivetkit | 3.0.0 |
| rivetkit-client | 3.0.0 |
| CPUs | 16 |
| Docker | 29.0.1 on Docker Desktop, kernel 6.6.87.2-microsoft-standard-WSL2 |
| Memory | 7.4 GiB |

## Invariants

| ID | Invariant | Result | Checked | Violations |
| --- | --- | --- | ---: | ---: |
| I1 | Durability | PASS | 2590 | 0 |
| I2 | No phantom writes | PASS | 2630 | 0 |
| I3 | No duplicates | PASS | 2630 | 0 |
| I4 | Real-time visibility | PASS | 282 | 0 |
| I5 | Recovery | PASS | 67 | 0 |
| I6 | Single activation | PASS | 4 | 0 |
| I7 | Workflow effects | PASS | 255 | 0 |
| I8 | Alarms | PASS | 128 | 0 |

I7: 1 tool calls ran more than once, each with a fault during its step.

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `storage` | s3 | 15.0 s | 35.7 s | 20.7 s |
| 2 | pause | engine-1 | 15.0 s | 30.1 s | 15.1 s |
| 3 | disconnect from `front` | node-a | 45.0 s | 65.3 s | 20.3 s |
| 4 | disconnect from `peer` | engine-2 | 75.0 s | 90.4 s | 15.3 s |
| 5 | kill-in-step | node-b | 101.8 s | 117.7 s | 15.9 s |
| 6 | replace | engine-3 | 125.0 s | 139.3 s | 14.3 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 14 | 4 | 0.5 s | 3.7 s | 24.3 s |
| 2 | 14 | 0 | 6.1 s | 9.3 s | 24.3 s |
| 3 | 9 | 0 | 6.5 s | 10.0 s | 26.9 s |
| 4 | 12 | 8 | -4.6 s | 13.1 s | 17.2 s |
| 5 | 9 | 0 | 36.8 s | 38.8 s | 54.4 s |
| 6 | 9 | 1 | 15.2 s | 17.2 s | 29.3 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 832 | 832 | 0 | 0 | 100.00% | 7.1 ms | 11.8 ms | 28.3 ms |
| During faults | 270 | 133 | 0 | 137 | 49.26% | 7.8 ms | 4706.3 ms | 4775.9 ms |
| After faults | 1665 | 1625 | 0 | 40 | 97.60% | 7.5 ms | 1076.2 ms | 4931.6 ms |
| All | 2767 | 2590 | 0 | 177 | 93.60% | 7.3 ms | 974.3 ms | 4931.6 ms |

Reads during the load: 272 succeeded out of 284.

Final reads: 10 succeeded out of 10 attempts, over 0.0 s.

## Sessions

4 sessions were sent 161 turns: 124 acknowledged, 37 with an unknown outcome, 0 failed.

## Errors

| Count | Error |
| ---: | --- |
| 184 | `timed out after 5000 ms` |
| 21 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 13 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 4 | `queue send failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_m` |
| 2 | `action failed: 502 Bad Gateway` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=2&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e03425` |
| 1 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
