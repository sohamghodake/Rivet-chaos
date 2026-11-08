# node-isolation-s2-1793840509

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `node-isolation` |
| Seed | 2 |
| Started | 2026-11-05 01:03:47 UTC |
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
| I1 | Durability | PASS | 26189 | 0 |
| I2 | No phantom writes | PASS | 26298 | 0 |
| I3 | No duplicates | PASS | 26298 | 0 |
| I4 | Real-time visibility | PASS | 2968 | 0 |
| I5 | Recovery | PASS | 31 | 0 |
| I6 | Single activation | FAIL | 10 | 9 |
| I7 | Workflow effects | PASS | 1716 | 0 |
| I8 | Alarms | PASS | 880 | 0 |

### I6 violations

- session/1 ran on node-a and node-b at once for 22.6 s from 119.2 s
- session/2 ran on node-a and node-b at once for 22.6 s from 119.2 s
- session/3 ran on node-a and node-b at once for 22.6 s from 119.2 s
- session/4 ran on node-a and node-b at once for 22.5 s from 119.4 s
- session/5 ran on node-a and node-b at once for 22.4 s from 119.3 s
- session/6 ran on node-a and node-b at once for 22.4 s from 119.4 s
- session/7 ran on node-a and node-b at once for 22.6 s from 119.2 s
- session/8 ran on node-a and node-b at once for 22.5 s from 119.2 s
- session/9 ran on node-a and node-b at once for 22.5 s from 119.3 s

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `front` | node-b | 60.0 s | 120.7 s | 60.7 s |
| 2 | disconnect from `front` | node-b | 210.0 s | 270.7 s | 60.6 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 31 | 30 | -1.6 s | 0.0 s | 60.3 s |
| 2 | 0 | - | - | - | - |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6457 | 6457 | 0 | 0 | 100.00% | 7.1 ms | 14.4 ms | 85.2 ms |
| During faults | 7132 | 6969 | 0 | 163 | 97.71% | 7.5 ms | 22.0 ms | 2639.9 ms |
| After faults | 12763 | 12763 | 0 | 0 | 100.00% | 7.5 ms | 20.0 ms | 114.2 ms |
| All | 26352 | 26189 | 0 | 163 | 99.38% | 7.4 ms | 19.7 ms | 2639.9 ms |

Reads during the load: 2918 succeeded out of 2932.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 898 turns: 835 acknowledged, 63 with an unknown outcome, 0 failed.

The sink recorded 1716 tool calls for 1716 distinct steps.

880 alarms fired, after their due time by p50 7.3 ms and at most 58719.5 ms.

## Errors

| Count | Error |
| ---: | --- |
| 219 | `timed out after 5000 ms` |
| 16 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 4 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 1 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
