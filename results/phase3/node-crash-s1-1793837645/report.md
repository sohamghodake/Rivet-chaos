# node-crash-s1-1793837645

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `node-crash` |
| Seed | 1 |
| Started | 2026-11-05 00:16:02 UTC |
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
| I1 | Durability | PASS | 23239 | 0 |
| I2 | No phantom writes | PASS | 23471 | 0 |
| I3 | No duplicates | PASS | 23471 | 0 |
| I4 | Real-time visibility | PASS | 2578 | 0 |
| I5 | Recovery | PASS | 66 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1728 | 0 |
| I8 | Alarms | PASS | 898 | 0 |

I8: 1 alarms fired on time by the wall clock but early by the monotonic clock, as a forward wall-clock adjustment causes.

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | node-a | 60.0 s | 90.8 s | 30.8 s |
| 2 | kill | node-b | 180.0 s | 210.8 s | 30.8 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 24 | 0 | 12.9 s | 14.1 s | 44.5 s |
| 2 | 42 | 0 | 13.2 s | 15.7 s | 46.1 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6561 | 6561 | 0 | 0 | 100.00% | 7.1 ms | 13.1 ms | 55.7 ms |
| During faults | 391 | 206 | 0 | 185 | 52.69% | 7.4 ms | 12.1 ms | 30.9 ms |
| After faults | 16528 | 16472 | 0 | 56 | 99.66% | 7.3 ms | 17.6 ms | 4792.3 ms |
| All | 23480 | 23239 | 0 | 241 | 98.97% | 7.2 ms | 16.8 ms | 4792.3 ms |

Reads during the load: 2528 succeeded out of 2549.

Final reads: 50 succeeded out of 51 attempts, over 1.1 s.

## Sessions

10 sessions were sent 898 turns: 843 acknowledged, 55 with an unknown outcome, 0 failed.

The sink recorded 1728 tool calls for 1728 distinct steps.

898 alarms fired, after their due time by p50 7.1 ms and at most 43222.9 ms.

## Errors

| Count | Error |
| ---: | --- |
| 307 | `timed out after 5000 ms` |
| 10 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 1 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
