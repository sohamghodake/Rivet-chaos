# node-isolation-s2-1794158511

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `node-isolation` |
| Seed | 2 |
| Started | 2026-11-08 17:23:38 UTC |
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
| I1 | Durability | PASS | 25625 | 0 |
| I2 | No phantom writes | PASS | 25729 | 0 |
| I3 | No duplicates | PASS | 25729 | 0 |
| I4 | Real-time visibility | PASS | 2899 | 0 |
| I5 | Recovery | PASS | 25 | 0 |
| I6 | Single activation | FAIL | 10 | 3 |
| I7 | Workflow effects | PASS | 1885 | 0 |
| I8 | Alarms | PASS | 961 | 0 |

### I6 violations

- session/2 ran on node-a and node-b at once for 214.6 s from 126.3 s
- session/4 ran on node-a and node-b at once for 201.9 s from 126.3 s
- session/6 ran on node-a and node-b at once for 214.6 s from 126.3 s

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `front` | node-b | 60.0 s | 120.5 s | 60.5 s |
| 2 | disconnect from `front` | node-b | 210.0 s | 270.5 s | 60.5 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 25 | 0 | 5.6 s | 6.4 s | 66.6 s |
| 2 | 0 | - | - | - | - |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6485 | 6485 | 0 | 0 | 100.00% | 6.7 ms | 11.6 ms | 108.9 ms |
| During faults | 6980 | 6803 | 0 | 177 | 97.46% | 6.9 ms | 18.6 ms | 72.3 ms |
| After faults | 12337 | 12337 | 0 | 0 | 100.00% | 6.8 ms | 13.7 ms | 4595.7 ms |
| All | 25802 | 25625 | 0 | 177 | 99.31% | 6.8 ms | 14.8 ms | 4595.7 ms |

Reads during the load: 2849 succeeded out of 2864.

Final reads: 50 succeeded out of 51 attempts, over 1.1 s.

## Sessions

10 sessions were sent 970 turns: 946 acknowledged, 24 with an unknown outcome, 0 failed.

The sink recorded 1885 tool calls for 1885 distinct steps.

961 alarms fired, after their due time by p50 6.7 ms and at most 65644.1 ms.

## Errors

| Count | Error |
| ---: | --- |
| 198 | `timed out after 5000 ms` |
| 16 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 2 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 1 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
