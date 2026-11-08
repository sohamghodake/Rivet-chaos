# node-isolation-s1-1794158055

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `node-isolation` |
| Seed | 1 |
| Started | 2026-11-08 17:15:35 UTC |
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
| I1 | Durability | PASS | 18422 | 0 |
| I2 | No phantom writes | PASS | 18650 | 0 |
| I3 | No duplicates | PASS | 18650 | 0 |
| I4 | Real-time visibility | PASS | 2021 | 0 |
| I5 | Recovery | PASS | 95 | 0 |
| I6 | Single activation | FAIL | 10 | 14 |
| I7 | Workflow effects | PASS | 1392 | 0 |
| I8 | Alarms | PASS | 735 | 0 |

### I6 violations

- session/0 ran on node-a and node-b at once for 203.9 s from 125.5 s
- session/0 ran on node-a and node-b at once for 9.7 s from 330.4 s
- session/1 ran on node-a and node-b at once for 54.1 s from 275.7 s
- session/2 ran on node-a and node-b at once for 214.6 s from 125.5 s
- session/3 ran on node-a and node-b at once for 203.6 s from 125.5 s
- session/3 ran on node-a and node-b at once for 9.8 s from 330.4 s
- session/4 ran on node-a and node-b at once for 53.3 s from 275.8 s
- session/5 ran on node-a and node-b at once for 64.3 s from 275.7 s
- session/6 ran on node-a and node-b at once for 203.9 s from 125.4 s
- session/6 ran on node-a and node-b at once for 9.7 s from 330.4 s
- session/7 ran on node-a and node-b at once for 203.5 s from 125.5 s
- session/7 ran on node-a and node-b at once for 9.7 s from 330.5 s
- session/8 ran on node-a and node-b at once for 214.6 s from 125.5 s
- session/9 ran on node-a and node-b at once for 54.1 s from 275.7 s

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `front` | node-a | 60.0 s | 120.6 s | 60.6 s |
| 2 | disconnect from `front` | node-b | 210.0 s | 270.7 s | 60.6 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 35 | 0 | 4.8 s | 6.2 s | 66.1 s |
| 2 | 60 | 0 | 5.1 s | 7.7 s | 67.9 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6485 | 6485 | 0 | 0 | 100.00% | 8.6 ms | 20.1 ms | 108.2 ms |
| During faults | 519 | 145 | 0 | 374 | 27.94% | 8.1 ms | 4816.9 ms | 4837.4 ms |
| After faults | 11792 | 11792 | 0 | 0 | 100.00% | 7.1 ms | 18.4 ms | 4624.4 ms |
| All | 18796 | 18422 | 0 | 374 | 98.01% | 7.6 ms | 19.5 ms | 4837.4 ms |

Reads during the load: 1971 succeeded out of 2005.

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Sessions

10 sessions were sent 783 turns: 655 acknowledged, 128 with an unknown outcome, 0 failed.

The sink recorded 1392 tool calls for 1392 distinct steps.

735 alarms fired, after their due time by p50 7.4 ms and at most 64944.7 ms.

## Errors

| Count | Error |
| ---: | --- |
| 490 | `timed out after 5000 ms` |
| 32 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 14 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
