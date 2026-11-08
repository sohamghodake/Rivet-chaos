# node-isolation-s4-1794159470

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `node-isolation` |
| Seed | 4 |
| Started | 2026-11-08 17:39:07 UTC |
| Engine image | `rivetdev/engine:3.0.0@sha256:5ae3c24275eeff87ad1962f4d66fcdb85786fc1397feeeb55afc2ca1f7ccf6f2` |
| rivet-chaos commit | 6388a11 |
| rivetkit | 3.0.0 |
| rivetkit-client | 3.0.0 |
| CPUs | 16 |
| Docker | 29.0.1 on Docker Desktop, kernel 6.6.87.2-microsoft-standard-WSL2 |
| Memory | 7.4 GiB |

## Invariants

| ID | Invariant | Result | Checked | Violations |
| --- | --- | --- | ---: | ---: |
| I1 | Durability | PASS | 18556 | 0 |
| I2 | No phantom writes | PASS | 18787 | 0 |
| I3 | No duplicates | PASS | 18787 | 0 |
| I4 | Real-time visibility | PASS | 2036 | 0 |
| I5 | Recovery | PASS | 93 | 0 |
| I6 | Single activation | FAIL | 10 | 19 |
| I7 | Workflow effects | PASS | 1450 | 0 |
| I8 | Alarms | PASS | 744 | 0 |

### I6 violations

- session/0 ran on node-a and node-b at once for 71.6 s from 268.6 s
- session/1 ran on node-a and node-b at once for 0.5 s from 125.0 s
- session/1 ran on node-a and node-b at once for 60.1 s from 268.6 s
- session/1 ran on node-a and node-b at once for 9.7 s from 330.5 s
- session/2 ran on node-a and node-b at once for 65.1 s from 275.2 s
- session/3 ran on node-a and node-b at once for 65.1 s from 275.2 s
- session/4 ran on node-a and node-b at once for 0.3 s from 125.0 s
- session/4 ran on node-a and node-b at once for 54.3 s from 275.1 s
- session/4 ran on node-a and node-b at once for 9.7 s from 330.5 s
- session/5 ran on node-a and node-b at once for 71.6 s from 268.6 s
- session/6 ran on node-a and node-b at once for 0.3 s from 125.0 s
- session/6 ran on node-a and node-b at once for 71.6 s from 268.6 s
- session/7 ran on node-a and node-b at once for 0.6 s from 125.0 s
- session/7 ran on node-a and node-b at once for 65.0 s from 275.1 s
- session/8 ran on node-a and node-b at once for 0.3 s from 125.0 s
- session/8 ran on node-a and node-b at once for 60.6 s from 268.6 s
- session/8 ran on node-a and node-b at once for 9.7 s from 330.5 s
- session/9 ran on node-a and node-b at once for 0.5 s from 125.0 s
- session/9 ran on node-a and node-b at once for 71.6 s from 268.6 s

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `front` | node-a | 60.0 s | 120.4 s | 60.4 s |
| 2 | disconnect from `front` | node-b | 210.0 s | 270.6 s | 60.6 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 34 | 0 | 4.5 s | 7.9 s | 65.8 s |
| 2 | 59 | 25 | 3.8 s | 6.2 s | 66.4 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6515 | 6515 | 0 | 0 | 100.00% | 6.9 ms | 16.2 ms | 68.3 ms |
| During faults | 548 | 193 | 0 | 355 | 35.22% | 8.1 ms | 4894.7 ms | 4980.7 ms |
| After faults | 11848 | 11848 | 0 | 0 | 100.00% | 7.9 ms | 29.8 ms | 4300.8 ms |
| All | 18911 | 18556 | 0 | 355 | 98.12% | 7.5 ms | 27.8 ms | 4980.7 ms |

Reads during the load: 1986 succeeded out of 2024.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 789 turns: 659 acknowledged, 130 with an unknown outcome, 0 failed.

The sink recorded 1450 tool calls for 1450 distinct steps.

744 alarms fired, after their due time by p50 7.6 ms and at most 64208.8 ms.

## Errors

| Count | Error |
| ---: | --- |
| 476 | `timed out after 5000 ms` |
| 32 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 15 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
