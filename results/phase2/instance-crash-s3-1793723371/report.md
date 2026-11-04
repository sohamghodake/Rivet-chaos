# instance-crash-s3-1793723371

**Verdict: PASS**

| | |
| --- | --- |
| Scenario | `instance-crash` |
| Seed | 3 |
| Started | 2026-11-03 16:30:37 UTC |
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
| I1 | Durability | PASS | 28763 | 0 |
| I2 | No phantom writes | PASS | 28786 | 0 |
| I3 | No duplicates | PASS | 28786 | 0 |

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | kill | engine-1 | 60.0 s | 92.4 s | 32.4 s |
| 2 | kill | engine-2 | 180.0 s | 212.3 s | 32.3 s |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6973 | 6970 | 0 | 3 | 99.96% | 6.4 ms | 19.8 ms | 2083.8 ms |
| During faults | 1194 | 1006 | 0 | 188 | 84.25% | 6.5 ms | 2054.1 ms | 4640.8 ms |
| After faults | 20788 | 20787 | 0 | 1 | 100.00% | 6.6 ms | 13.0 ms | 3618.8 ms |
| All | 28955 | 28763 | 0 | 192 | 99.34% | 6.5 ms | 14.4 ms | 4640.8 ms |

Final reads: 50 succeeded out of 50 attempts, over 0.2 s.

## Errors

| Count | Error |
| ---: | --- |
| 143 | `timed out after 5000 ms` |
| 47 | `action failed (guard/service_unavailable): Service unavailable (depot_endpoint_unresolved). Retry after 100 ms., metadata=Some(Object {"reason": String("depot_endpoint_unresolved"), "retry_after_ms": ` |
| 1 | `action failed (guard/actor_stopped_while_waiting): Actor stopped before accepting the request., metadata=None` |
| 1 | `error sending request for url (http://localhost:6420/gateway/ledger/action/append?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=29&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=d5515e0342` |
