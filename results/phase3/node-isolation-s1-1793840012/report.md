# node-isolation-s1-1793840012

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `node-isolation` |
| Seed | 1 |
| Started | 2026-11-05 00:55:22 UTC |
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
| I1 | Durability | PASS | 20234 | 0 |
| I2 | No phantom writes | PASS | 20461 | 0 |
| I3 | No duplicates | PASS | 20461 | 0 |
| I4 | Real-time visibility | PASS | 2211 | 0 |
| I5 | Recovery | PASS | 95 | 0 |
| I6 | Single activation | FAIL | 10 | 10 |
| I7 | Workflow effects | PASS | 1545 | 0 |
| I8 | Alarms | FAIL | 800 | 1 |

### I6 violations

- session/0 ran on node-a and node-b at once for 72.8 s from 267.2 s
- session/1 ran on node-a and node-b at once for 62.0 s from 267.1 s
- session/2 ran on node-a and node-b at once for 222.6 s from 117.6 s
- session/3 ran on node-a and node-b at once for 72.8 s from 267.2 s
- session/4 ran on node-a and node-b at once for 72.9 s from 267.1 s
- session/5 ran on node-a and node-b at once for 222.6 s from 117.4 s
- session/6 ran on node-a and node-b at once for 62.5 s from 267.4 s
- session/7 ran on node-a and node-b at once for 222.6 s from 117.5 s
- session/8 ran on node-a and node-b at once for 222.6 s from 117.4 s
- session/9 ran on node-a and node-b at once for 222.6 s from 117.4 s

### I8 violations

- alarm 8/56 never fired

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | disconnect from `front` | node-a | 60.0 s | 120.5 s | 60.5 s |
| 2 | disconnect from `front` | node-b | 210.0 s | 270.6 s | 60.6 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 36 | 36 | -3.1 s | -1.7 s | 57.8 s |
| 2 | 59 | 59 | -3.4 s | -1.6 s | 58.2 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6558 | 6558 | 0 | 0 | 100.00% | 7.2 ms | 13.4 ms | 24.8 ms |
| During faults | 1195 | 883 | 0 | 312 | 73.89% | 7.8 ms | 2330.6 ms | 2558.0 ms |
| After faults | 12793 | 12793 | 0 | 0 | 100.00% | 7.5 ms | 16.0 ms | 47.2 ms |
| All | 20546 | 20234 | 0 | 312 | 98.48% | 7.4 ms | 16.4 ms | 2558.0 ms |

Reads during the load: 2161 succeeded out of 2201.

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Sessions

10 sessions were sent 830 turns: 725 acknowledged, 105 with an unknown outcome, 0 failed.

The sink recorded 1545 tool calls for 1545 distinct steps.

799 alarms fired, after their due time by p50 7.2 ms and at most 56577.3 ms.

## Errors

| Count | Error |
| ---: | --- |
| 413 | `timed out after 5000 ms` |
| 32 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 12 | `queue send failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
