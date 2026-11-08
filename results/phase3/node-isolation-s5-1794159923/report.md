# node-isolation-s5-1794159923

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `node-isolation` |
| Seed | 5 |
| Started | 2026-11-08 17:46:37 UTC |
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
| I1 | Durability | PASS | 26377 | 0 |
| I2 | No phantom writes | PASS | 26491 | 0 |
| I3 | No duplicates | PASS | 26491 | 0 |
| I4 | Real-time visibility | PASS | 2947 | 0 |
| I5 | Recovery | PASS | 26 | 0 |
| I6 | Single activation | FAIL | 10 | 1 |
| I7 | Workflow effects | PASS | 1945 | 0 |
| I8 | Alarms | PASS | 987 | 0 |

### I6 violations

- session/6 ran on node-a and node-b at once for 209.9 s from 118.3 s

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `front` | node-b | 60.0 s | 120.5 s | 60.5 s |
| 2 | disconnect from `front` | node-b | 210.0 s | 270.6 s | 60.6 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 26 | 26 | -2.3 s | -1.2 s | 58.8 s |
| 2 | 0 | - | - | - | - |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6495 | 6495 | 0 | 0 | 100.00% | 6.7 ms | 14.6 ms | 109.3 ms |
| During faults | 7207 | 7047 | 0 | 160 | 97.78% | 6.9 ms | 17.2 ms | 3211.3 ms |
| After faults | 12835 | 12835 | 0 | 0 | 100.00% | 7.2 ms | 17.9 ms | 109.0 ms |
| All | 26537 | 26377 | 0 | 160 | 99.40% | 7.0 ms | 17.4 ms | 3211.3 ms |

Reads during the load: 2897 succeeded out of 2914.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 989 turns: 982 acknowledged, 7 with an unknown outcome, 0 failed.

The sink recorded 1945 tool calls for 1945 distinct steps.

987 alarms fired, after their due time by p50 6.8 ms and at most 37.9 ms.

## Errors

| Count | Error |
| ---: | --- |
| 167 | `timed out after 5000 ms` |
| 16 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 1 | `action failed (core/internal_error): An internal error occurred, metadata=None` |
