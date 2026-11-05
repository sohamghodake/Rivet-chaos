# node-crash-s3-1793838626

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `node-crash` |
| Seed | 3 |
| Started | 2026-11-05 00:32:14 UTC |
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
| I1 | Durability | PASS | 23134 | 0 |
| I2 | No phantom writes | PASS | 23359 | 0 |
| I3 | No duplicates | PASS | 23359 | 0 |
| I4 | Real-time visibility | PASS | 2556 | 0 |
| I5 | Recovery | PASS | 55 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1819 | 0 |
| I8 | Alarms | PASS | 927 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | node-a | 60.0 s | 90.8 s | 30.7 s |
| 2 | kill | node-a | 180.0 s | 210.8 s | 30.7 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 30 | 0 | 13.8 s | 14.6 s | 45.1 s |
| 2 | 25 | 0 | 13.8 s | 14.2 s | 44.9 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6494 | 6494 | 0 | 0 | 100.00% | 7.1 ms | 14.2 ms | 107.6 ms |
| During faults | 385 | 209 | 0 | 176 | 54.29% | 7.4 ms | 17.0 ms | 53.7 ms |
| After faults | 16485 | 16431 | 0 | 54 | 99.67% | 7.3 ms | 16.8 ms | 3104.8 ms |
| All | 23364 | 23134 | 0 | 230 | 99.02% | 7.3 ms | 16.3 ms | 3104.8 ms |

Reads during the load: 2505 succeeded out of 2535.

Final reads: 51 succeeded out of 51 attempts, over 30.2 s.

## Sessions

10 sessions were sent 927 turns: 887 acknowledged, 40 with an unknown outcome, 0 failed.

The sink recorded 1819 tool calls for 1819 distinct steps.

927 alarms fired, after their due time by p50 7.1 ms and at most 44323.0 ms.

## Errors

| Count | Error |
| ---: | --- |
| 296 | `timed out after 5000 ms` |
| 4 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
