# instance-crash-s6-1793724589

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-crash` |
| Seed | 6 |
| Started | 2026-11-03 16:50:55 UTC |
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
| I1 | Durability | PASS | 26220 | 0 |
| I2 | No phantom writes | PASS | 26272 | 0 |
| I3 | No duplicates | PASS | 26272 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-2 | 60.0 s | 93.1 s | 33.0 s |
| 2 | kill | engine-2 | 180.0 s | 214.3 s | 34.2 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6877 | 6871 | 0 | 6 | 99.91% | 7.8 ms | 25.1 ms | 2083.8 ms |
| During faults | 343 | 91 | 0 | 252 | 26.53% | 13.1 ms | 4948.0 ms | 4948.0 ms |
| After faults | 19262 | 19258 | 0 | 4 | 99.98% | 9.0 ms | 48.4 ms | 3407.9 ms |
| All | 26482 | 26220 | 0 | 262 | 99.01% | 8.6 ms | 44.2 ms | 4948.0 ms |

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Errors

| Count | Error |
| ---: | --- |
| 194 | `timed out after 5000 ms` |
| 34 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 6 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 6 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 110 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(110)})` |
| 5 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 3 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 133 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(133)})` |
| 3 | `action failed (sqlite/closed): An internal error occurred, metadata=None` |
| 2 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 135 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(135)})` |
| 1 | `action failed (core/internal_error): An internal error occurred, metadata=Some(Object {"error": String("actor response aborted")})` |
| 1 | `action failed (depot/overloaded): The storage host refused the command (transport_window); retry after 100 ms., metadata=None` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 120 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(120)})` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 147 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(147)})` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 150 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(150)})` |
| 1 | `action failed (guard/service_unavailable): Service unavailable (ups_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("ups_endpoint_unresolved"), "retry_after_ms": Numb` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=18&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e0342` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=41&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e0342` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=42&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e0342` |
