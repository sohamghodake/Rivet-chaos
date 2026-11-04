# instance-crash-s4-1793723776

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-crash` |
| Seed | 4 |
| Started | 2026-11-03 16:37:22 UTC |
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
| I1 | Durability | PASS | 28269 | 0 |
| I2 | No phantom writes | PASS | 28347 | 0 |
| I3 | No duplicates | PASS | 28347 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-1 | 60.0 s | 93.3 s | 33.3 s |
| 2 | kill | engine-3 | 180.0 s | 213.1 s | 33.1 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6876 | 6868 | 0 | 8 | 99.88% | 7.6 ms | 22.8 ms | 2097.2 ms |
| During faults | 1396 | 1218 | 0 | 178 | 87.25% | 7.7 ms | 3076.1 ms | 4907.0 ms |
| After faults | 20184 | 20183 | 0 | 1 | 100.00% | 7.5 ms | 15.7 ms | 4010.0 ms |
| All | 28456 | 28269 | 0 | 187 | 99.34% | 7.5 ms | 19.2 ms | 4907.0 ms |

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Errors

| Count | Error |
| ---: | --- |
| 162 | `timed out after 5000 ms` |
| 9 | `action failed (guard/service_unavailable): Service unavailable (ups_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("ups_endpoint_unresolved"), "retry_after_ms": Numb` |
| 6 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 5 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 1 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 1 | `action failed (sqlite/closed): An internal error occurred, metadata=None` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=17&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e0342` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=6&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e03425` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=7&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e03425` |
