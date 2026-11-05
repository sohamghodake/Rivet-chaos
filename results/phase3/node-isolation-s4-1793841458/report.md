# node-isolation-s4-1793841458

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `node-isolation` |
| Seed | 4 |
| Started | 2026-11-05 01:18:53 UTC |
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
| I1 | Durability | PASS | 20165 | 0 |
| I2 | No phantom writes | PASS | 20391 | 0 |
| I3 | No duplicates | PASS | 20391 | 0 |
| I4 | Real-time visibility | PASS | 2208 | 0 |
| I5 | Recovery | PASS | 99 | 0 |
| I6 | Single activation | FAIL | 10 | 10 |
| I7 | Workflow effects | PASS | 1353 | 0 |
| I8 | Alarms | FAIL | 689 | 6 |

I8: 1 alarms were booked more than once because a fault interrupted their booking step; each booking fired once.

I8: 40 alarms fired on time by the wall clock but early by the monotonic clock, as a forward wall-clock adjustment causes.

### I6 violations

- session/0 ran on node-a and node-b at once for 180.6 s from 117.7 s
- session/1 ran on node-a and node-b at once for 30.3 s from 268.0 s
- session/2 ran on node-a and node-b at once for 180.9 s from 117.6 s
- session/3 ran on node-a and node-b at once for 180.6 s from 117.7 s
- session/4 ran on node-a and node-b at once for 180.9 s from 117.6 s
- session/5 ran on node-a and node-b at once for 30.5 s from 267.8 s
- session/6 ran on node-a and node-b at once for 30.6 s from 267.8 s
- session/7 ran on node-a and node-b at once for 180.9 s from 117.6 s
- session/8 ran on node-a and node-b at once for 180.8 s from 117.5 s
- session/9 ran on node-a and node-b at once for 180.8 s from 117.6 s

### I8 violations

- alarm 0/64 was booked 2 times, due at 269.4 s, 329.5 s, with no fault during its booking step
- alarm 1/73 was booked 2 times, due at 269.4 s, 329.5 s, with no fault during its booking step
- alarm 2/28 was booked 2 times, due at 119.2 s, 179.2 s, with no fault during its booking step
- alarm 4/25 was booked 2 times, due at 119.2 s, 179.3 s, with no fault during its booking step
- alarm 4/50 was booked 2 times, due at 269.4 s, 329.5 s, with no fault during its booking step
- alarm 8/26 was booked 2 times, due at 119.2 s, 179.3 s, with no fault during its booking step

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `front` | node-a | 60.0 s | 120.5 s | 60.4 s |
| 2 | disconnect from `front` | node-b | 210.0 s | 270.5 s | 60.5 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 39 | 38 | -2.9 s | 57.8 s | 115.7 s |
| 2 | 60 | 60 | -2.6 s | -1.1 s | 59.2 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6515 | 6515 | 0 | 0 | 100.00% | 7.1 ms | 13.7 ms | 49.0 ms |
| During faults | 1054 | 741 | 0 | 313 | 70.30% | 8.2 ms | 3160.1 ms | 3432.4 ms |
| After faults | 12909 | 12909 | 0 | 0 | 100.00% | 7.3 ms | 14.8 ms | 44.6 ms |
| All | 20478 | 20165 | 0 | 313 | 98.47% | 7.2 ms | 15.4 ms | 3432.4 ms |

Reads during the load: 2158 succeeded out of 2197.

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Sessions

10 sessions were sent 726 turns: 564 acknowledged, 162 with an unknown outcome, 0 failed.

The sink recorded 1353 tool calls for 1353 distinct steps.

697 alarms fired, after their due time by p50 7.1 ms and at most 148944.3 ms.

## Errors

| Count | Error |
| ---: | --- |
| 468 | `timed out after 5000 ms` |
| 32 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 14 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
