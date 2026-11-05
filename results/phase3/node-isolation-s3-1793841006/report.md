# node-isolation-s3-1793841006

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `node-isolation` |
| Seed | 3 |
| Started | 2026-11-05 01:11:20 UTC |
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
| I1 | Durability | PASS | 26470 | 0 |
| I2 | No phantom writes | PASS | 26584 | 0 |
| I3 | No duplicates | PASS | 26584 | 0 |
| I4 | Real-time visibility | PASS | 2968 | 0 |
| I5 | Recovery | PASS | 33 | 0 |
| I6 | Single activation | FAIL | 10 | 7 |
| I7 | Workflow effects | PASS | 1785 | 0 |
| I8 | Alarms | PASS | 913 | 0 |

### I6 violations

- session/1 ran on node-a and node-b at once for 24.4 s from 116.8 s
- session/3 ran on node-a and node-b at once for 24.1 s from 116.9 s
- session/4 ran on node-a and node-b at once for 24.4 s from 116.8 s
- session/5 ran on node-a and node-b at once for 24.2 s from 116.8 s
- session/7 ran on node-a and node-b at once for 24.4 s from 116.9 s
- session/8 ran on node-a and node-b at once for 24.2 s from 116.9 s
- session/9 ran on node-a and node-b at once for 24.2 s from 116.9 s

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `front` | node-a | 60.0 s | 120.5 s | 60.5 s |
| 2 | disconnect from `front` | node-a | 210.0 s | 270.5 s | 60.4 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 33 | 33 | -3.8 s | -2.2 s | 58.1 s |
| 2 | 0 | - | - | - | - |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6501 | 6501 | 0 | 0 | 100.00% | 7.1 ms | 13.9 ms | 43.0 ms |
| During faults | 7279 | 7123 | 0 | 156 | 97.86% | 7.3 ms | 17.2 ms | 1607.7 ms |
| After faults | 12846 | 12846 | 0 | 0 | 100.00% | 7.2 ms | 16.4 ms | 106.9 ms |
| All | 26626 | 26470 | 0 | 156 | 99.41% | 7.2 ms | 16.2 ms | 1607.7 ms |

Reads during the load: 2918 succeeded out of 2938.

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Sessions

10 sessions were sent 927 turns: 878 acknowledged, 49 with an unknown outcome, 0 failed.

The sink recorded 1785 tool calls for 1785 distinct steps.

913 alarms fired, after their due time by p50 7.0 ms and at most 56352.0 ms.

## Errors

| Count | Error |
| ---: | --- |
| 202 | `timed out after 5000 ms` |
| 16 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 7 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
