# node-crash-s5-1793839561

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `node-crash` |
| Seed | 5 |
| Started | 2026-11-05 00:47:15 UTC |
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
| I1 | Durability | PASS | 23163 | 0 |
| I2 | No phantom writes | PASS | 23398 | 0 |
| I3 | No duplicates | PASS | 23398 | 0 |
| I4 | Real-time visibility | PASS | 2602 | 0 |
| I5 | Recovery | PASS | 53 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1875 | 0 |
| I8 | Alarms | PASS | 955 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | node-b | 60.0 s | 90.7 s | 30.7 s |
| 2 | kill | node-b | 180.0 s | 210.9 s | 30.9 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 31 | 0 | 13.2 s | 14.6 s | 44.8 s |
| 2 | 22 | 0 | 13.1 s | 14.5 s | 45.1 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6481 | 6481 | 0 | 0 | 100.00% | 7.1 ms | 13.7 ms | 40.7 ms |
| During faults | 374 | 196 | 0 | 178 | 52.41% | 7.5 ms | 16.7 ms | 33.4 ms |
| After faults | 16546 | 16486 | 0 | 60 | 99.64% | 7.3 ms | 15.5 ms | 2961.4 ms |
| All | 23401 | 23163 | 0 | 238 | 98.98% | 7.3 ms | 15.0 ms | 2961.4 ms |

Reads during the load: 2552 succeeded out of 2575.

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Sessions

10 sessions were sent 955 turns: 930 acknowledged, 25 with an unknown outcome, 0 failed.

The sink recorded 1875 tool calls for 1875 distinct steps.

955 alarms fired, after their due time by p50 7.1 ms and at most 43787.5 ms.

## Errors

| Count | Error |
| ---: | --- |
| 281 | `timed out after 5000 ms` |
| 5 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
