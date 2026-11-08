# instance-pause-s2-1794153881

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-pause` |
| Seed | 2 |
| Started | 2026-11-08 16:06:00 UTC |
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
| I1 | Durability | PASS | 24134 | 0 |
| I2 | No phantom writes | PASS | 24135 | 0 |
| I3 | No duplicates | PASS | 24135 | 0 |
| I4 | Real-time visibility | PASS | 2711 | 0 |
| I5 | Recovery | PASS | 50 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1878 | 0 |
| I8 | Alarms | PASS | 956 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | pause | engine-3 | 60.1 s | 105.3 s | 45.2 s |
| 2 | pause | engine-3 | 180.1 s | 225.3 s | 45.2 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 24 | 24 | -7.3 s | -4.4 s | 40.3 s |
| 2 | 26 | 26 | -5.1 s | -2.1 s | 41.5 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6430 | 6430 | 0 | 0 | 100.00% | 6.9 ms | 18.0 ms | 407.0 ms |
| During faults | 1823 | 1625 | 0 | 198 | 89.14% | 7.5 ms | 1156.1 ms | 3491.8 ms |
| After faults | 16079 | 16079 | 0 | 0 | 100.00% | 7.6 ms | 27.0 ms | 188.4 ms |
| All | 24332 | 24134 | 0 | 198 | 99.19% | 7.4 ms | 27.5 ms | 3491.8 ms |

Reads during the load: 2661 succeeded out of 2691.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 975 turns: 956 acknowledged, 19 with an unknown outcome, 0 failed.

The sink recorded 1878 tool calls for 1878 distinct steps.

956 alarms fired, after their due time by p50 7.2 ms and at most 87.0 ms.

## Errors

| Count | Error |
| ---: | --- |
| 242 | `timed out after 5000 ms` |
| 5 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
