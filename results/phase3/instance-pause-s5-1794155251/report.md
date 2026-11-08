# instance-pause-s5-1794155251

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-pause` |
| Seed | 5 |
| Started | 2026-11-08 16:28:43 UTC |
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
| I1 | Durability | PASS | 23290 | 0 |
| I2 | No phantom writes | PASS | 23376 | 0 |
| I3 | No duplicates | PASS | 23376 | 0 |
| I4 | Real-time visibility | PASS | 2616 | 0 |
| I5 | Recovery | PASS | 80 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1650 | 0 |
| I8 | Alarms | PASS | 837 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | pause | engine-3 | 60.1 s | 105.2 s | 45.1 s |
| 2 | pause | engine-2 | 180.1 s | 225.2 s | 45.1 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 25 | 25 | -12.7 s | -4.4 s | 40.0 s |
| 2 | 55 | 0 | 1.7 s | 5.4 s | 50.3 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6479 | 6479 | 0 | 0 | 100.00% | 7.1 ms | 15.8 ms | 82.3 ms |
| During faults | 1108 | 882 | 0 | 226 | 79.60% | 7.7 ms | 2971.6 ms | 4980.7 ms |
| After faults | 15929 | 15929 | 0 | 0 | 100.00% | 7.6 ms | 29.2 ms | 283.9 ms |
| All | 23516 | 23290 | 0 | 226 | 99.04% | 7.5 ms | 26.6 ms | 4980.7 ms |

Reads during the load: 2566 succeeded out of 2587.

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Sessions

10 sessions were sent 877 turns: 809 acknowledged, 68 with an unknown outcome, 0 failed.

The sink recorded 1650 tool calls for 1650 distinct steps.

837 alarms fired, after their due time by p50 7.4 ms and at most 95.6 ms.

## Errors

| Count | Error |
| ---: | --- |
| 312 | `timed out after 5000 ms` |
| 3 | `action failed (depot/request_deadline_exceeded): The command deadline elapsed; its outcome may be unknown., metadata=None` |
