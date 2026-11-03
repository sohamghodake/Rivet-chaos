# instance-crash-s9-1793725818

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-crash` |
| Seed | 9 |
| Started | 2026-11-03 17:11:26 UTC |
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
| I1 | Durability | PASS | 28139 | 0 |
| I2 | No phantom writes | PASS | 28176 | 0 |
| I3 | No duplicates | PASS | 28176 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-1 | 60.0 s | 92.6 s | 32.6 s |
| 2 | kill | engine-1 | 180.0 s | 213.0 s | 33.0 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6978 | 6968 | 0 | 10 | 99.86% | 7.3 ms | 14.8 ms | 2073.6 ms |
| During faults | 644 | 422 | 0 | 222 | 65.53% | 8.3 ms | 1849.3 ms | 4722.7 ms |
| After faults | 20750 | 20749 | 0 | 1 | 100.00% | 7.5 ms | 14.3 ms | 410.9 ms |
| All | 28372 | 28139 | 0 | 233 | 99.18% | 7.5 ms | 15.1 ms | 4722.7 ms |

Final reads: 50 succeeded out of 50 attempts, over 1.0 s.

## Errors

| Count | Error |
| ---: | --- |
| 161 | `timed out after 5000 ms` |
| 57 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 6 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 3 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 2 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 1 | `action failed (core/internal_error): An internal error occurred, metadata=Some(Object {"error": String("actor response aborted")})` |
| 1 | `action failed (sqlite/closed): An internal error occurred, metadata=None` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=10&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e0342` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=46&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e0342` |
