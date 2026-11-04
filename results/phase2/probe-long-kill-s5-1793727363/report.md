# probe-long-kill-s5-1793727363

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `probe-long-kill` |
| Seed | 5 |
| Started | 2026-11-03 17:37:14 UTC |
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
| I1 | Durability | PASS | 14206 | 0 |
| I2 | No phantom writes | PASS | 14319 | 0 |
| I3 | No duplicates | PASS | 14319 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-3 | 60.1 s | 243.2 s | 183.2 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6869 | 6865 | 0 | 4 | 99.94% | 7.4 ms | 23.5 ms | 2175.0 ms |
| During faults | 1936 | 1389 | 0 | 547 | 71.75% | 7.6 ms | 33.3 ms | 4997.1 ms |
| After faults | 5960 | 5952 | 0 | 8 | 99.87% | 7.6 ms | 16.1 ms | 4337.7 ms |
| All | 14765 | 14206 | 0 | 559 | 96.21% | 7.5 ms | 21.2 ms | 4997.1 ms |

Final reads: 50 succeeded out of 50 attempts, over 1.1 s.

## Errors

| Count | Error |
| ---: | --- |
| 537 | `timed out after 5000 ms` |
| 13 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 3 | `action failed (guard/service_unavailable): Service unavailable (ups_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("ups_endpoint_unresolved"), "retry_after_ms": Numb` |
| 2 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 2 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 1 | `action failed: 502 Bad Gateway` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=30&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e0342` |
