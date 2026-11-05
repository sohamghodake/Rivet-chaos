# kill-in-step-s2-1793849634

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `kill-in-step` |
| Seed | 2 |
| Started | 2026-11-05 03:35:09 UTC |
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
| I1 | Durability | PASS | 13795 | 0 |
| I2 | No phantom writes | PASS | 14250 | 0 |
| I3 | No duplicates | PASS | 14250 | 0 |
| I4 | Real-time visibility | PASS | 1603 | 0 |
| I5 | Recovery | PASS | 132 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1536 | 0 |
| I8 | Alarms | PASS | 790 | 0 |

I7: 4 tool calls ran more than once, each with a fault during its step.

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill-in-step | node-a | 60.3 s | 90.9 s | 30.7 s |
| 2 | kill-in-step | node-b | 120.3 s | 150.9 s | 30.7 s |
| 3 | kill-in-step | node-b | 180.2 s | 210.8 s | 30.6 s |
| 4 | kill-in-step | node-a | 240.2 s | 271.0 s | 30.7 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 24 | 0 | 13.2 s | 14.9 s | 45.1 s |
| 2 | 42 | 0 | 13.5 s | 16.5 s | 46.8 s |
| 3 | 30 | 0 | 13.4 s | 15.2 s | 45.5 s |
| 4 | 36 | 0 | 13.4 s | 14.7 s | 45.1 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6486 | 6486 | 0 | 0 | 100.00% | 7.0 ms | 15.1 ms | 47.9 ms |
| During faults | 722 | 358 | 0 | 364 | 49.58% | 7.3 ms | 14.0 ms | 21.3 ms |
| After faults | 7061 | 6951 | 0 | 110 | 98.44% | 7.4 ms | 108.4 ms | 3813.4 ms |
| All | 14269 | 13795 | 0 | 474 | 96.68% | 7.2 ms | 21.7 ms | 3813.4 ms |

Reads during the load: 1553 succeeded out of 1603.

Final reads: 50 succeeded out of 50 attempts, over 0.1 s.

## Sessions

10 sessions were sent 791 turns: 672 acknowledged, 119 with an unknown outcome, 0 failed.

The sink recorded 1540 tool calls for 1536 distinct steps.

790 alarms fired, after their due time by p50 7.0 ms and at most 43850.0 ms.

## Errors

| Count | Error |
| ---: | --- |
| 629 | `timed out after 5000 ms` |
| 13 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 1 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
