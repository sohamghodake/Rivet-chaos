# instance-down-long-s5-1793830494

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-down-long` |
| Seed | 5 |
| Started | 2026-11-04 22:16:07 UTC |
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
| I1 | Durability | PASS | 15010 | 0 |
| I2 | No phantom writes | PASS | 15183 | 0 |
| I3 | No duplicates | PASS | 15183 | 0 |
| I4 | Real-time visibility | PASS | 1758 | 0 |
| I5 | Recovery | PASS | 51 | 0 |
| I6 | Single activation | PASS | 10 | 0 |
| I7 | Workflow effects | PASS | 1796 | 0 |
| I8 | Alarms | PASS | 914 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-3 | 60.0 s | 242.7 s | 182.6 s |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 51 | 45 | -144.8 s | 1.8 s | 183.6 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6489 | 6489 | 0 | 0 | 100.00% | 7.0 ms | 13.5 ms | 42.2 ms |
| During faults | 2891 | 2452 | 0 | 439 | 84.81% | 6.6 ms | 918.5 ms | 4747.3 ms |
| After faults | 6069 | 6069 | 0 | 0 | 100.00% | 6.8 ms | 19.3 ms | 452.4 ms |
| All | 15449 | 15010 | 0 | 439 | 97.16% | 6.8 ms | 17.8 ms | 4747.3 ms |

Reads during the load: 1708 succeeded out of 1759.

Final reads: 50 succeeded out of 50 attempts, over 0.1 s.

## Sessions

10 sessions were sent 928 turns: 889 acknowledged, 39 with an unknown outcome, 0 failed.

The sink recorded 1796 tool calls for 1796 distinct steps.

914 alarms fired, after their due time by p50 6.7 ms and at most 43027.5 ms.

## Errors

| Count | Error |
| ---: | --- |
| 527 | `timed out after 5000 ms` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=40&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=…): client` |
