# node-crash-s4-1793839111

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `node-crash` |
| Seed | 4 |
| Started | 2026-11-05 00:39:45 UTC |
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
| I1 | Durability | PASS | 23091 | 0 |
| I2 | No phantom writes | PASS | 23318 | 0 |
| I3 | No duplicates | PASS | 23318 | 0 |
| I4 | Real-time visibility | PASS | 2505 | 0 |
| I5 | Recovery | PASS | 67 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1732 | 0 |
| I8 | Alarms | PASS | 883 | 0 |

I8: 1 alarms fired on time by the wall clock but early by the monotonic clock, as a forward wall-clock adjustment causes.

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | node-a | 60.0 s | 90.7 s | 30.7 s |
| 2 | kill | node-b | 180.0 s | 210.7 s | 30.7 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 31 | 0 | 13.8 s | 15.3 s | 45.8 s |
| 2 | 36 | 0 | 13.9 s | 16.5 s | 46.9 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6513 | 6513 | 0 | 0 | 100.00% | 7.1 ms | 13.2 ms | 28.0 ms |
| During faults | 311 | 136 | 0 | 175 | 43.73% | 7.5 ms | 11.5 ms | 26.5 ms |
| After faults | 16500 | 16442 | 0 | 58 | 99.65% | 7.3 ms | 17.0 ms | 3627.0 ms |
| All | 23324 | 23091 | 0 | 233 | 99.00% | 7.3 ms | 15.8 ms | 3627.0 ms |

Reads during the load: 2454 succeeded out of 2483.

Final reads: 51 succeeded out of 51 attempts, over 30.2 s.

## Sessions

10 sessions were sent 883 turns: 817 acknowledged, 66 with an unknown outcome, 0 failed.

The sink recorded 1732 tool calls for 1732 distinct steps.

883 alarms fired, after their due time by p50 7.2 ms and at most 44097.3 ms.

## Errors

| Count | Error |
| ---: | --- |
| 322 | `timed out after 5000 ms` |
| 6 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
