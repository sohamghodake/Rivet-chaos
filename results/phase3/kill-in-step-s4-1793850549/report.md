# kill-in-step-s4-1793850549

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `kill-in-step` |
| Seed | 4 |
| Started | 2026-11-05 03:51:00 UTC |
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
| I1 | Durability | PASS | 14566 | 0 |
| I2 | No phantom writes | PASS | 15012 | 0 |
| I3 | No duplicates | PASS | 15012 | 0 |
| I4 | Real-time visibility | PASS | 1596 | 0 |
| I5 | Recovery | PASS | 126 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1425 | 0 |
| I8 | Alarms | PASS | 734 | 0 |

I7: 4 tool calls ran more than once, each with a fault during its step.

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill-in-step | node-b | 60.1 s | 90.9 s | 30.7 s |
| 2 | kill-in-step | node-b | 120.8 s | 151.5 s | 30.7 s |
| 3 | kill-in-step | node-a | 180.4 s | 211.1 s | 30.7 s |
| 4 | kill-in-step | node-a | 240.2 s | 270.9 s | 30.7 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 35 | 0 | 12.6 s | 14.5 s | 45.1 s |
| 2 | 22 | 0 | 11.8 s | 12.2 s | 42.6 s |
| 3 | 45 | 0 | 12.4 s | 15.3 s | 45.8 s |
| 4 | 24 | 0 | 12.6 s | 14.8 s | 43.3 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6526 | 6526 | 0 | 0 | 100.00% | 7.1 ms | 14.8 ms | 29.7 ms |
| During faults | 869 | 508 | 0 | 361 | 58.46% | 7.4 ms | 15.5 ms | 36.1 ms |
| After faults | 7629 | 7532 | 0 | 97 | 98.73% | 7.4 ms | 39.8 ms | 4935.7 ms |
| All | 15024 | 14566 | 0 | 458 | 96.95% | 7.2 ms | 19.4 ms | 4935.7 ms |

Reads during the load: 1545 succeeded out of 1591.

Final reads: 51 succeeded out of 51 attempts, over 30.2 s.

## Sessions

10 sessions were sent 734 turns: 584 acknowledged, 150 with an unknown outcome, 0 failed.

The sink recorded 1429 tool calls for 1425 distinct steps.

734 alarms fired, after their due time by p50 7.1 ms and at most 43035.3 ms.

## Errors

| Count | Error |
| ---: | --- |
| 640 | `timed out after 5000 ms` |
| 14 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
