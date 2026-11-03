# smoke-s1-1793722321

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `smoke` |
| Seed | 1 |
| Started | 2026-11-03 16:13:35 UTC |
| Engine image | `rivetdev/engine:3.0.0@sha256:5ae3c24275eeff87ad1962f4d66fcdb85786fc1397feeeb55afc2ca1f7ccf6f2` |
| rivet-chaos commit | none |
| rivetkit | 3.0.0 |
| rivetkit-client | 3.0.0 |
| CPUs | 16 |
| Docker | 29.0.1 on Docker Desktop, kernel 6.6.87.2-microsoft-standard-WSL2 |
| Memory | 7.4 GiB |

## Invariants

| ID | Invariant | Result | Checked | Violations |
| --- | --- | --- | ---: | ---: |
| I1 | Durability | PASS | 2458 | 0 |
| I2 | No phantom writes | PASS | 2474 | 0 |
| I3 | No duplicates | PASS | 2474 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-2 | 20.0 s | 37.3 s | 17.2 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 1078 | 1073 | 0 | 5 | 99.54% | 6.6 ms | 25.6 ms | 2111.5 ms |
| During faults | 67 | 43 | 0 | 24 | 64.18% | 7.5 ms | 2390.0 ms | 2390.0 ms |
| After faults | 1342 | 1342 | 0 | 0 | 100.00% | 6.6 ms | 10.7 ms | 109.8 ms |
| All | 2487 | 2458 | 0 | 29 | 98.83% | 6.6 ms | 25.3 ms | 2390.0 ms |

Final reads: 10 succeeded out of 10 attempts, over 0.0 s.

## Errors

| Count | Error |
| ---: | --- |
| 24 | `timed out after 5000 ms` |
| 3 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 2 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
