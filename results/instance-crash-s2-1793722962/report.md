# instance-crash-s2-1793722962

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-crash` |
| Seed | 2 |
| Started | 2026-11-03 16:23:47 UTC |
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
| I1 | Durability | PASS | 31400 | 0 |
| I2 | No phantom writes | PASS | 31418 | 0 |
| I3 | No duplicates | PASS | 31418 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-3 | 60.0 s | 92.7 s | 32.7 s |
| 2 | kill | engine-3 | 180.0 s | 212.7 s | 32.7 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 7003 | 6995 | 0 | 8 | 99.89% | 6.8 ms | 18.9 ms | 2067.5 ms |
| During faults | 3670 | 3547 | 0 | 123 | 96.65% | 6.5 ms | 31.1 ms | 4796.4 ms |
| After faults | 20859 | 20858 | 0 | 1 | 100.00% | 6.9 ms | 15.0 ms | 599.0 ms |
| All | 31532 | 31400 | 0 | 132 | 99.58% | 6.9 ms | 16.0 ms | 4796.4 ms |

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Errors

| Count | Error |
| ---: | --- |
| 88 | `timed out after 5000 ms` |
| 39 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 4 | `action failed (guard/service_unavailable): Service unavailable (depot_overloaded). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_overloaded"), "retry_after_ms": Number(100)})` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=46&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e0342` |
