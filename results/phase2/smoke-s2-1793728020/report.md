# smoke-s2-1793728020

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `smoke` |
| Seed | 2 |
| Started | 2026-11-03 17:48:15 UTC |
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
| I1 | Durability | PASS | 2022 | 0 |
| I2 | No phantom writes | PASS | 2054 | 0 |
| I3 | No duplicates | PASS | 2054 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-2 | 20.1 s | 38.0 s | 17.9 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 1030 | 1026 | 0 | 4 | 99.61% | 9.4 ms | 96.2 ms | 2844.7 ms |
| During faults | 39 | 4 | 0 | 35 | 10.26% | 10.8 ms | 19.7 ms | 19.7 ms |
| After faults | 992 | 992 | 0 | 0 | 100.00% | 8.5 ms | 40.4 ms | 3170.3 ms |
| All | 2061 | 2022 | 0 | 39 | 98.11% | 8.9 ms | 88.7 ms | 3170.3 ms |

Final reads: 10 succeeded out of 10 attempts, over 0.0 s.

## Errors

| Count | Error |
| ---: | --- |
| 32 | `timed out after 5000 ms` |
| 4 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 1 | `action failed (ups/transport): Worker transport failed; the operation may have executed., metadata=None` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=6&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e03425` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=7&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e03425` |
