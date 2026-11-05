# kill-in-step-s1-1793849176

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `kill-in-step` |
| Seed | 1 |
| Started | 2026-11-05 03:27:31 UTC |
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
| I1 | Durability | PASS | 14595 | 0 |
| I2 | No phantom writes | PASS | 15031 | 0 |
| I3 | No duplicates | PASS | 15031 | 0 |
| I4 | Real-time visibility | PASS | 1613 | 0 |
| I5 | Recovery | PASS | 113 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1552 | 0 |
| I8 | Alarms | PASS | 813 | 0 |

I7: 4 tool calls ran more than once, each with a fault during its step.

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill-in-step | node-b | 60.1 s | 90.8 s | 30.7 s |
| 2 | kill-in-step | node-a | 120.5 s | 151.3 s | 30.7 s |
| 3 | kill-in-step | node-a | 180.2 s | 210.9 s | 30.7 s |
| 4 | kill-in-step | node-a | 240.0 s | 270.7 s | 30.7 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 25 | 0 | 11.9 s | 13.9 s | 44.5 s |
| 2 | 42 | 0 | 12.2 s | 14.2 s | 44.2 s |
| 3 | 28 | 0 | 12.1 s | 14.5 s | 45.1 s |
| 4 | 18 | 0 | 11.9 s | 13.3 s | 44.0 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6580 | 6580 | 0 | 0 | 100.00% | 6.9 ms | 13.9 ms | 32.9 ms |
| During faults | 830 | 476 | 0 | 354 | 57.35% | 7.3 ms | 19.7 ms | 42.2 ms |
| After faults | 7632 | 7539 | 0 | 93 | 98.78% | 7.1 ms | 20.4 ms | 4898.8 ms |
| All | 15042 | 14595 | 0 | 447 | 97.03% | 7.0 ms | 16.3 ms | 4898.8 ms |

Reads during the load: 1563 succeeded out of 1615.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 815 turns: 709 acknowledged, 106 with an unknown outcome, 0 failed.

The sink recorded 1556 tool calls for 1552 distinct steps.

813 alarms fired, after their due time by p50 6.9 ms and at most 42636.8 ms.

## Errors

| Count | Error |
| ---: | --- |
| 593 | `timed out after 5000 ms` |
| 11 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 1 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
