# node-isolation-s5-1793841921

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `node-isolation` |
| Seed | 5 |
| Started | 2026-11-05 01:27:12 UTC |
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
| I1 | Durability | PASS | 26348 | 0 |
| I2 | No phantom writes | PASS | 26462 | 0 |
| I3 | No duplicates | PASS | 26462 | 0 |
| I4 | Real-time visibility | PASS | 2939 | 0 |
| I5 | Recovery | PASS | 27 | 0 |
| I6 | Single activation | FAIL | 10 | 4 |
| I7 | Workflow effects | PASS | 1861 | 0 |
| I8 | Alarms | PASS | 945 | 0 |

### I6 violations

- session/4 ran on node-a and node-b at once for 221.9 s from 118.2 s
- session/5 ran on node-a and node-b at once for 211.7 s from 118.3 s
- session/8 ran on node-a and node-b at once for 211.4 s from 118.3 s
- session/9 ran on node-a and node-b at once for 211.1 s from 118.3 s

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `front` | node-b | 60.0 s | 120.5 s | 60.5 s |
| 2 | disconnect from `front` | node-b | 210.0 s | 270.5 s | 60.5 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 27 | 27 | -2.3 s | -0.5 s | 58.8 s |
| 2 | 0 | - | - | - | - |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6478 | 6478 | 0 | 0 | 100.00% | 7.2 ms | 13.8 ms | 23.9 ms |
| During faults | 7185 | 7027 | 0 | 158 | 97.80% | 7.3 ms | 18.9 ms | 2480.1 ms |
| After faults | 12843 | 12843 | 0 | 0 | 100.00% | 7.3 ms | 16.7 ms | 74.9 ms |
| All | 26506 | 26348 | 0 | 158 | 99.40% | 7.3 ms | 16.6 ms | 2480.1 ms |

Reads during the load: 2889 succeeded out of 2907.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 953 turns: 925 acknowledged, 28 with an unknown outcome, 0 failed.

The sink recorded 1861 tool calls for 1861 distinct steps.

945 alarms fired, after their due time by p50 7.0 ms and at most 57109.0 ms.

## Errors

| Count | Error |
| ---: | --- |
| 186 | `timed out after 5000 ms` |
| 16 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 2 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
