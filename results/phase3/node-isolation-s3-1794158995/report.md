# node-isolation-s3-1794158995

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `node-isolation` |
| Seed | 3 |
| Started | 2026-11-08 17:31:42 UTC |
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
| I1 | Durability | PASS | 25503 | 0 |
| I2 | No phantom writes | PASS | 25604 | 0 |
| I3 | No duplicates | PASS | 25604 | 0 |
| I4 | Real-time visibility | PASS | 2868 | 0 |
| I5 | Recovery | PASS | 29 | 0 |
| I6 | Single activation | FAIL | 10 | 6 |
| I7 | Workflow effects | PASS | 1773 | 0 |
| I8 | Alarms | PASS | 903 | 0 |

I8: 1 alarms were booked more than once because a fault interrupted their booking step; each booking fired once.

I8: 1 alarms fired early by the wall clock but on time by the monotonic clock, as a backward wall-clock step between booking and firing causes.

### I6 violations

- session/1 ran on node-a and node-b at once for 40.0 s from 126.7 s
- session/2 ran on node-a and node-b at once for 40.2 s from 126.6 s
- session/3 ran on node-a and node-b at once for 40.2 s from 126.6 s
- session/6 ran on node-a and node-b at once for 40.2 s from 126.6 s
- session/7 ran on node-a and node-b at once for 40.2 s from 126.6 s
- session/9 ran on node-a and node-b at once for 40.2 s from 126.6 s

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `front` | node-a | 60.0 s | 120.5 s | 60.5 s |
| 2 | disconnect from `front` | node-a | 210.0 s | 270.4 s | 60.4 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 29 | 0 | 6.0 s | 7.6 s | 67.3 s |
| 2 | 0 | - | - | - | - |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6528 | 6528 | 0 | 0 | 100.00% | 6.6 ms | 11.8 ms | 51.1 ms |
| During faults | 6873 | 6697 | 0 | 176 | 97.44% | 6.8 ms | 28.5 ms | 172.2 ms |
| After faults | 12278 | 12278 | 0 | 0 | 100.00% | 6.7 ms | 14.4 ms | 4452.4 ms |
| All | 25679 | 25503 | 0 | 176 | 99.31% | 6.7 ms | 14.9 ms | 4452.4 ms |

Reads during the load: 2818 succeeded out of 2834.

Final reads: 50 succeeded out of 50 attempts, over 0.5 s.

## Sessions

10 sessions were sent 921 turns: 873 acknowledged, 48 with an unknown outcome, 0 failed.

The sink recorded 1773 tool calls for 1773 distinct steps.

904 alarms fired, after their due time by p50 6.7 ms and at most 66024.5 ms.

## Errors

| Count | Error |
| ---: | --- |
| 219 | `timed out after 5000 ms` |
| 16 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 5 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
