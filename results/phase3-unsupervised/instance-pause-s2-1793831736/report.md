# instance-pause-s2-1793831736

**Verdict: FAIL**

| | |
| --- | --- |
| Scenario | `instance-pause` |
| Seed | 2 |
| Started | 2026-11-04 22:37:23 UTC |
| Engine image | `rivetdev/engine:3.0.0@sha256:5ae3c24275eeff87ad1962f4d66fcdb85786fc1397feeeb55afc2ca1f7ccf6f2` |
| rivet-chaos commit | 9f5be2f |
| rivetkit | 3.0.0 |
| rivetkit-client | 3.0.0 |
| CPUs | 16 |
| Docker | 29.0.1 on Docker Desktop, kernel 6.6.87.2-microsoft-standard-WSL2 |
| Memory | 7.4 GiB |

## Invariants

| ID | Invariant | Result | Checked | Violations |
| --- | --- | --- | ---: | ---: |
| I1 | Durability | UNKNOWN | 2331 | 0 |
| I2 | No phantom writes | UNKNOWN | 2331 | 0 |
| I3 | No duplicates | UNKNOWN | 2331 | 0 |
| I4 | Real-time visibility | PASS | 3266 | 0 |
| I5 | Recovery | FAIL | 28 | 47 |
| I6 | Single activation | UNKNOWN | 0 | 0 |
| I7 | Workflow effects | UNKNOWN | 0 | 0 |
| I8 | Alarms | UNKNOWN | 0 | 0 |

46 ledgers had no successful read after their last append, so they were not checked: ledger/0, ledger/1, ledger/10, ledger/11, ledger/12, ledger/13, ledger/14, ledger/15, ledger/17, ledger/18, ledger/19, ledger/2, ledger/20, ledger/21, ledger/22, ledger/23, ledger/25, ledger/26, ledger/27, ledger/28, ledger/29, ledger/3, ledger/30, ledger/31, ledger/32, ledger/33, ledger/34, ledger/35, ledger/36, ledger/37, ledger/38, ledger/39, ledger/40, ledger/41, ledger/42, ledger/43, ledger/45, ledger/46, ledger/47, ledger/48, ledger/49, ledger/5, ledger/6, ledger/7, ledger/8, ledger/9

I6: The run has no sink records to check.

I7: The run has no sink records to check.

I8: The run has no sink records to check.

### I5 violations

- pause engine-3 started at 180.0 s and never healed
- ledger/0 was never read back after its last append
- ledger/1 was never read back after its last append
- ledger/10 was never read back after its last append
- ledger/11 was never read back after its last append
- ledger/12 was never read back after its last append
- ledger/13 was never read back after its last append
- ledger/14 was never read back after its last append
- ledger/15 was never read back after its last append
- ledger/17 was never read back after its last append
- ledger/18 was never read back after its last append
- ledger/19 was never read back after its last append
- ledger/2 was never read back after its last append
- ledger/20 was never read back after its last append
- ledger/21 was never read back after its last append
- ledger/22 was never read back after its last append
- ledger/23 was never read back after its last append
- ledger/25 was never read back after its last append
- ledger/26 was never read back after its last append
- ledger/27 was never read back after its last append
- ledger/28 was never read back after its last append
- ledger/29 was never read back after its last append
- ledger/3 was never read back after its last append
- ledger/30 was never read back after its last append
- ledger/31 was never read back after its last append
- ledger/32 was never read back after its last append
- ledger/33 was never read back after its last append
- ledger/34 was never read back after its last append
- ledger/35 was never read back after its last append
- ledger/36 was never read back after its last append
- ledger/37 was never read back after its last append
- ledger/38 was never read back after its last append
- ledger/39 was never read back after its last append
- ledger/40 was never read back after its last append
- ledger/41 was never read back after its last append
- ledger/42 was never read back after its last append
- ledger/43 was never read back after its last append
- ledger/45 was never read back after its last append
- ledger/46 was never read back after its last append
- ledger/47 was never read back after its last append
- ledger/48 was never read back after its last append
- ledger/49 was never read back after its last append
- ledger/5 was never read back after its last append
- ledger/6 was never read back after its last append
- ledger/7 was never read back after its last append
- ledger/8 was never read back after its last append
- ledger/9 was never read back after its last append

## Faults

| # | Fault | Target | Start | Heal | Length |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | pause | engine-3 | 60.0 s | 105.1 s | 45.1 s |
| 2 | pause | engine-3 | 180.0 s | never | - |

## Recovery

Actors whose calls failed during a fault, and when each answered again, relative to the heal. A negative time means it answered while the fault was still in place.

| # | Actors interrupted | Back before the heal | After heal, p50 | After heal, max | Longest outage |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 28 | 28 | -14.7 s | -11.5 s | 33.5 s |
| 2 | 0 | - | - | - | - |

## Appends

Latency is for acknowledged appends, from invoke to result. A call belongs to the window it was invoked in.

| Window | Calls | OK | Fail | Info | Succeeded | p50 | p99 | Max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before faults | 6473 | 6473 | 0 | 0 | 100.00% | 7.0 ms | 12.3 ms | 23.5 ms |
| During faults | 14698 | 14613 | 0 | 85 | 99.42% | 6.6 ms | 15.6 ms | 143.4 ms |
| After faults | 8161 | 8161 | 0 | 0 | 100.00% | 6.6 ms | 11.7 ms | 109.6 ms |
| All | 29332 | 29247 | 0 | 85 | 99.71% | 6.7 ms | 14.0 ms | 143.4 ms |

Reads during the load: 3266 succeeded out of 3275.

## Sessions

10 sessions were sent 972 turns: 950 acknowledged, 22 with an unknown outcome, 0 failed.

## Errors

| Count | Error |
| ---: | --- |
| 116 | `timed out after 5000 ms` |
