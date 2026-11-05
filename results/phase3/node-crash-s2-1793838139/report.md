# node-crash-s2-1793838139

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `node-crash` |
| Seed | 2 |
| Started | 2026-11-05 00:24:09 UTC |
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
| I1 | Durability | PASS | 23372 | 0 |
| I2 | No phantom writes | PASS | 23593 | 0 |
| I3 | No duplicates | PASS | 23593 | 0 |
| I4 | Real-time visibility | PASS | 2624 | 0 |
| I5 | Recovery | PASS | 48 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1751 | 0 |
| I8 | Alarms | PASS | 896 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | node-b | 60.0 s | 90.7 s | 30.7 s |
| 2 | kill | node-b | 180.0 s | 210.8 s | 30.8 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 27 | 0 | 13.0 s | 14.5 s | 45.1 s |
| 2 | 21 | 0 | 13.0 s | 14.4 s | 45.1 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6454 | 6454 | 0 | 0 | 100.00% | 7.3 ms | 13.8 ms | 47.9 ms |
| During faults | 509 | 333 | 0 | 176 | 65.42% | 7.4 ms | 11.9 ms | 33.4 ms |
| After faults | 16634 | 16585 | 0 | 49 | 99.71% | 7.5 ms | 17.0 ms | 4980.7 ms |
| All | 23597 | 23372 | 0 | 225 | 99.05% | 7.4 ms | 16.3 ms | 4980.7 ms |

Reads during the load: 2574 succeeded out of 2601.

Final reads: 50 succeeded out of 51 attempts, over 1.1 s.

## Sessions

10 sessions were sent 896 turns: 836 acknowledged, 60 with an unknown outcome, 0 failed.

The sink recorded 1751 tool calls for 1751 distinct steps.

896 alarms fired, after their due time by p50 7.2 ms and at most 43387.7 ms.

## Errors

| Count | Error |
| ---: | --- |
| 307 | `timed out after 5000 ms` |
| 5 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 1 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
