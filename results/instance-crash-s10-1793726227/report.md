# instance-crash-s10-1793726227

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-crash` |
| Seed | 10 |
| Started | 2026-11-03 17:18:14 UTC |
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
| I1 | Durability | PASS | 26995 | 0 |
| I2 | No phantom writes | PASS | 27067 | 0 |
| I3 | No duplicates | PASS | 27067 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-3 | 60.0 s | 92.9 s | 32.9 s |
| 2 | kill | engine-1 | 180.0 s | 213.3 s | 33.3 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6930 | 6923 | 0 | 7 | 99.90% | 7.4 ms | 21.3 ms | 2087.9 ms |
| During faults | 441 | 185 | 0 | 256 | 41.95% | 9.4 ms | 4886.5 ms | 4915.2 ms |
| After faults | 19888 | 19887 | 0 | 1 | 99.99% | 7.5 ms | 14.8 ms | 4825.1 ms |
| All | 27259 | 26995 | 0 | 264 | 99.03% | 7.5 ms | 16.8 ms | 4915.2 ms |

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Errors

| Count | Error |
| ---: | --- |
| 174 | `timed out after 5000 ms` |
| 57 | `action failed (ups/consumer_closed): The consumer closed without replying; the operation may have executed., metadata=None` |
| 19 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 6 | `action failed (guard/service_unavailable): Service unavailable (ups_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("ups_endpoint_unresolved"), "retry_after_ms": Numb` |
| 4 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 1 | `action failed (sqlite/closed): An internal error occurred, metadata=None` |
| 1 | `action failed (ups/transport): Worker transport failed; the operation may have executed., metadata=None` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=19&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e0342` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=35&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e0342` |
