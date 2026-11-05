# kill-in-step-s3-1793850091

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `kill-in-step` |
| Seed | 3 |
| Started | 2026-11-05 03:42:46 UTC |
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
| I1 | Durability | PASS | 14400 | 0 |
| I2 | No phantom writes | PASS | 14846 | 0 |
| I3 | No duplicates | PASS | 14846 | 0 |
| I4 | Real-time visibility | PASS | 1619 | 0 |
| I5 | Recovery | PASS | 108 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1519 | 0 |
| I8 | Alarms | PASS | 780 | 0 |

I7: 4 tool calls ran more than once, each with a fault during its step.

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill-in-step | node-a | 60.1 s | 90.8 s | 30.7 s |
| 2 | kill-in-step | node-a | 120.1 s | 150.9 s | 30.8 s |
| 3 | kill-in-step | node-a | 180.1 s | 210.9 s | 30.7 s |
| 4 | kill-in-step | node-b | 240.2 s | 270.9 s | 30.7 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 31 | 0 | 13.3 s | 14.5 s | 45.1 s |
| 2 | 20 | 0 | 13.2 s | 15.7 s | 45.8 s |
| 3 | 13 | 0 | 13.1 s | 14.4 s | 45.1 s |
| 4 | 44 | 0 | 13.4 s | 15.7 s | 45.7 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6521 | 6521 | 0 | 0 | 100.00% | 7.0 ms | 14.0 ms | 34.7 ms |
| During faults | 1032 | 672 | 0 | 360 | 65.12% | 7.4 ms | 13.3 ms | 27.2 ms |
| After faults | 7305 | 7207 | 0 | 98 | 98.66% | 7.2 ms | 31.0 ms | 4993.0 ms |
| All | 14858 | 14400 | 0 | 458 | 96.92% | 7.1 ms | 18.8 ms | 4993.0 ms |

Reads during the load: 1568 succeeded out of 1609.

Final reads: 51 succeeded out of 51 attempts, over 30.2 s.

## Sessions

10 sessions were sent 780 turns: 657 acknowledged, 123 with an unknown outcome, 0 failed.

The sink recorded 1523 tool calls for 1519 distinct steps.

780 alarms fired, after their due time by p50 7.0 ms and at most 43759.3 ms.

## Errors

| Count | Error |
| ---: | --- |
| 613 | `timed out after 5000 ms` |
| 9 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
