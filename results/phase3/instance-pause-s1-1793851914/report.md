# instance-pause-s1-1793851914

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-pause` |
| Seed | 1 |
| Started | 2026-11-05 04:14:15 UTC |
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
| I1 | Durability | PASS | 22083 | 0 |
| I2 | No phantom writes | PASS | 22173 | 0 |
| I3 | No duplicates | PASS | 22173 | 0 |
| I4 | Real-time visibility | PASS | 2434 | 0 |
| I5 | Recovery | PASS | 113 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1419 | 0 |
| I8 | Alarms | PASS | 746 | 0 |

I8: 1 alarms fired on time by the wall clock but early by the monotonic clock, as a forward wall-clock adjustment causes.

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | pause | engine-1 | 60.1 s | 105.3 s | 45.1 s |
| 2 | pause | engine-3 | 180.2 s | 225.4 s | 45.2 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 54 | 0 | 1.9 s | 4.2 s | 48.6 s |
| 2 | 59 | 0 | 1.7 s | 3.7 s | 47.7 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6498 | 6498 | 0 | 0 | 100.00% | 7.7 ms | 30.9 ms | 106.0 ms |
| During faults | 265 | 8 | 0 | 257 | 3.02% | 8.6 ms | 4988.9 ms | 4988.9 ms |
| After faults | 15579 | 15577 | 0 | 2 | 99.99% | 8.0 ms | 44.1 ms | 2061.3 ms |
| All | 22342 | 22083 | 0 | 259 | 98.84% | 8.0 ms | 40.0 ms | 4988.9 ms |

Reads during the load: 2384 succeeded out of 2413.

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Sessions

10 sessions were sent 804 turns: 693 acknowledged, 111 with an unknown outcome, 0 failed.

The sink recorded 1419 tool calls for 1419 distinct steps.

746 alarms fired, after their due time by p50 7.7 ms and at most 48251.6 ms.

## Errors

| Count | Error |
| ---: | --- |
| 380 | `timed out after 5000 ms` |
| 8 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 5 | `action failed (ups/transport): Worker transport failed; the operation may have executed., metadata=None` |
| 4 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 1 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 1 | `queue send failed (ups/transport): Worker transport failed; the operation may have executed., metadata=None` |
