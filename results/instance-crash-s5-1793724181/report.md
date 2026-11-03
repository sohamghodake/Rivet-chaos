# instance-crash-s5-1793724181

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-crash` |
| Seed | 5 |
| Started | 2026-11-03 16:44:08 UTC |
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
| I1 | Durability | PASS | 26567 | 0 |
| I2 | No phantom writes | PASS | 26655 | 0 |
| I3 | No duplicates | PASS | 26655 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-3 | 60.0 s | 92.7 s | 32.7 s |
| 2 | kill | engine-2 | 180.0 s | 212.9 s | 32.9 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6876 | 6867 | 0 | 9 | 99.87% | 7.5 ms | 25.2 ms | 2082.8 ms |
| During faults | 303 | 91 | 0 | 212 | 30.03% | 10.8 ms | 3858.4 ms | 3858.4 ms |
| After faults | 19615 | 19609 | 0 | 6 | 99.97% | 7.8 ms | 69.1 ms | 4325.4 ms |
| All | 26794 | 26567 | 0 | 227 | 99.15% | 7.7 ms | 64.0 ms | 4325.4 ms |

Final reads: 50 succeeded out of 51 attempts, over 1.5 s.

## Errors

| Count | Error |
| ---: | --- |
| 200 | `timed out after 5000 ms` |
| 9 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 6 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 6 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 1 | `action failed (core/internal_error): An internal error occurred, metadata=None` |
| 1 | `action failed (core/internal_error): An internal error occurred, metadata=Some(Object {"error": String("actor response aborted")})` |
| 1 | `action failed (depot/overloaded): The storage host refused the command (endpoint_unresolved); retry after 100 ms., metadata=None` |
| 1 | `action failed (sqlite/closed): An internal error occurred, metadata=None` |
| 1 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=25&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e0342` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=39&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e0342` |
