# smoke-s3-1793823326

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `smoke` |
| Seed | 3 |
| Started | 2026-11-04 20:16:48 UTC |
| Engine image | `rivetdev/engine:3.0.0@sha256:5ae3c24275eeff87ad1962f4d66fcdb85786fc1397feeeb55afc2ca1f7ccf6f2` |
| rivet-chaos commit | 47a596a |
| rivetkit | 3.0.0 |
| rivetkit-client | 3.0.0 |
| CPUs | 16 |
| Docker | 29.0.1 on Docker Desktop, kernel 6.6.87.2-microsoft-standard-WSL2 |
| Memory | 7.4 GiB |

## Invariants

| ID | Invariant | Result | Checked | Violations |
| --- | --- | --- | ---: | ---: |
| I1 | Durability | PASS | 2123 | 0 |
| I2 | No phantom writes | PASS | 2127 | 0 |
| I3 | No duplicates | PASS | 2127 | 0 |
| I4 | Real-time visibility | PASS | 261 | 0 |
| I5 | Recovery | PASS | 8 | 0 |
| I6 | Single activation | PASS | 3 | 0 |
| I7 | Workflow effects | PASS | 91 | 0 |
| I8 | Alarms | PASS | 43 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-2 | 20.0 s | 38.1 s | 18.1 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 8 | 0 | 1.8 s | 7.5 s | 22.3 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 1064 | 1064 | 0 | 0 | 100.00% | 7.4 ms | 13.5 ms | 16.0 ms |
| During faults | 62 | 37 | 0 | 25 | 59.68% | 7.9 ms | 4255.7 ms | 4255.7 ms |
| After faults | 1022 | 1022 | 0 | 0 | 100.00% | 7.4 ms | 15.3 ms | 1442.8 ms |
| All | 2148 | 2123 | 0 | 25 | 98.84% | 7.4 ms | 15.1 ms | 4255.7 ms |

Reads during the load: 251 succeeded out of 254.

Final reads: 10 succeeded out of 10 attempts, over 0.0 s.

## Sessions

3 sessions were sent 47 turns: 40 acknowledged, 7 with an unknown outcome, 0 failed.

## Errors

| Count | Error |
| ---: | --- |
| 31 | `timed out after 5000 ms` |
| 3 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 1 | `queue send failed (sqlite/transaction_closed): An internal error occurred, metadata=None` |
