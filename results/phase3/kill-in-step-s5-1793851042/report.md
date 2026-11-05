# kill-in-step-s5-1793851042

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `kill-in-step` |
| Seed | 5 |
| Started | 2026-11-05 03:59:23 UTC |
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
| I1 | Durability | PASS | 14466 | 0 |
| I2 | No phantom writes | PASS | 14919 | 0 |
| I3 | No duplicates | PASS | 14919 | 0 |
| I4 | Real-time visibility | PASS | 1714 | 0 |
| I5 | Recovery | PASS | 120 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1632 | 0 |
| I8 | Alarms | PASS | 836 | 0 |

I7: 4 tool calls ran more than once, each with a fault during its step.

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill-in-step | node-a | 60.4 s | 91.2 s | 30.8 s |
| 2 | kill-in-step | node-a | 120.0 s | 150.7 s | 30.7 s |
| 3 | kill-in-step | node-b | 180.2 s | 211.0 s | 30.8 s |
| 4 | kill-in-step | node-b | 240.0 s | 270.8 s | 30.7 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 33 | 0 | 11.7 s | 14.4 s | 45.1 s |
| 2 | 21 | 0 | 11.9 s | 12.9 s | 43.4 s |
| 3 | 42 | 0 | 11.9 s | 14.6 s | 45.3 s |
| 4 | 24 | 0 | 12.0 s | 14.7 s | 42.9 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6525 | 6525 | 0 | 0 | 100.00% | 7.1 ms | 14.5 ms | 31.5 ms |
| During faults | 732 | 374 | 0 | 358 | 51.09% | 7.4 ms | 15.1 ms | 49.2 ms |
| After faults | 7668 | 7567 | 0 | 101 | 98.68% | 7.4 ms | 29.7 ms | 4927.5 ms |
| All | 14925 | 14466 | 0 | 459 | 96.92% | 7.3 ms | 20.1 ms | 4927.5 ms |

Reads during the load: 1664 succeeded out of 1706.

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Sessions

10 sessions were sent 836 turns: 741 acknowledged, 95 with an unknown outcome, 0 failed.

The sink recorded 1636 tool calls for 1632 distinct steps.

836 alarms fired, after their due time by p50 7.0 ms and at most 42476.6 ms.

## Errors

| Count | Error |
| ---: | --- |
| 590 | `timed out after 5000 ms` |
| 6 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
