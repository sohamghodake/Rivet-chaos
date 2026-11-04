# instance-crash-s2-1793826661

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-crash` |
| Seed | 2 |
| Started | 2026-11-04 21:12:50 UTC |
| Engine image | `rivetdev/engine:3.0.0@sha256:5ae3c24275eeff87ad1962f4d66fcdb85786fc1397feeeb55afc2ca1f7ccf6f2` |
| rivet-chaos commit | 5740a28 |
| rivetkit | 3.0.0 |
| rivetkit-client | 3.0.0 |
| CPUs | 16 |
| Docker | 29.0.1 on Docker Desktop, kernel 6.6.87.2-microsoft-standard-WSL2 |
| Memory | 7.4 GiB |

## Invariants

| ID | Invariant | Result | Checked | Violations |
| --- | --- | --- | ---: | ---: |
| I1 | Durability | PASS | 25291 | 0 |
| I2 | No phantom writes | PASS | 25314 | 0 |
| I3 | No duplicates | PASS | 25314 | 0 |
| I4 | Real-time visibility | PASS | 2866 | 0 |
| I5 | Recovery | PASS | 39 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1880 | 0 |
| I8 | Alarms | PASS | 956 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-3 | 60.0 s | 93.6 s | 33.6 s |
| 2 | kill | engine-3 | 180.0 s | 213.5 s | 33.5 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 22 | 19 | -1.0 s | 2.2 s | 34.9 s |
| 2 | 17 | 17 | -1.8 s | -0.8 s | 32.5 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6445 | 6445 | 0 | 0 | 100.00% | 7.5 ms | 14.3 ms | 27.9 ms |
| During faults | 979 | 797 | 0 | 182 | 81.41% | 8.0 ms | 3362.8 ms | 4784.1 ms |
| After faults | 18049 | 18049 | 0 | 0 | 100.00% | 8.5 ms | 80.2 ms | 530.4 ms |
| All | 25473 | 25291 | 0 | 182 | 99.29% | 8.1 ms | 71.2 ms | 4784.1 ms |

Reads during the load: 2816 succeeded out of 2827.

Final reads: 50 succeeded out of 50 attempts, over 0.7 s.

## Sessions

10 sessions were sent 975 turns: 951 acknowledged, 24 with an unknown outcome, 0 failed.

The sink recorded 1880 tool calls for 1880 distinct steps.

956 alarms fired, after their due time by p50 7.6 ms and at most 349.6 ms.

## Errors

| Count | Error |
| ---: | --- |
| 153 | `timed out after 5000 ms` |
| 54 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 10 | `queue send failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_m` |
