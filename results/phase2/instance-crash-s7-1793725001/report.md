# instance-crash-s7-1793725001

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-crash` |
| Seed | 7 |
| Started | 2026-11-03 16:57:51 UTC |
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
| I1 | Durability | PASS | 27652 | 0 |
| I2 | No phantom writes | PASS | 27736 | 0 |
| I3 | No duplicates | PASS | 27736 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-3 | 60.1 s | 92.9 s | 32.8 s |
| 2 | kill | engine-1 | 180.1 s | 213.4 s | 33.3 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6931 | 6926 | 0 | 5 | 99.93% | 7.4 ms | 20.8 ms | 2148.4 ms |
| During faults | 700 | 500 | 0 | 200 | 71.43% | 8.6 ms | 2594.8 ms | 4151.3 ms |
| After faults | 20226 | 20226 | 0 | 0 | 100.00% | 7.7 ms | 20.3 ms | 3153.9 ms |
| All | 27857 | 27652 | 0 | 205 | 99.26% | 7.6 ms | 23.6 ms | 4151.3 ms |

Final reads: 50 succeeded out of 50 attempts, over 0.3 s.

## Errors

| Count | Error |
| ---: | --- |
| 174 | `timed out after 5000 ms` |
| 26 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 3 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 1 | `action failed: 502 Bad Gateway` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=27&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e0342` |
