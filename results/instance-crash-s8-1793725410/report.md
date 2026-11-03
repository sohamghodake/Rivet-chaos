# instance-crash-s8-1793725410

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-crash` |
| Seed | 8 |
| Started | 2026-11-03 17:04:38 UTC |
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
| I1 | Durability | PASS | 27540 | 0 |
| I2 | No phantom writes | PASS | 27564 | 0 |
| I3 | No duplicates | PASS | 27564 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-1 | 60.0 s | 92.6 s | 32.5 s |
| 2 | kill | engine-1 | 180.0 s | 212.9 s | 32.9 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6889 | 6883 | 0 | 6 | 99.91% | 7.5 ms | 19.3 ms | 2092.0 ms |
| During faults | 551 | 322 | 0 | 229 | 58.44% | 8.8 ms | 2707.5 ms | 4870.1 ms |
| After faults | 20336 | 20335 | 0 | 1 | 100.00% | 7.6 ms | 15.4 ms | 3792.9 ms |
| All | 27776 | 27540 | 0 | 236 | 99.15% | 7.5 ms | 16.8 ms | 4870.1 ms |

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Errors

| Count | Error |
| ---: | --- |
| 161 | `timed out after 5000 ms` |
| 68 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 2 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 1 | `action failed (depot/unavailable): The storage host could not finish the command (command_capacity); its outcome may be unknown., metadata=Some(Object {"reason": String("command_capacity")})` |
| 1 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=18&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e0342` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=22&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e0342` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=6&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e03425` |
