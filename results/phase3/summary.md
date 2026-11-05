| Scenario | Fault | Runs | Passed | I1 | I2 | I3 | I4 | I5 | I6 | I7 | I8 | Acknowledged appends | Actors interrupted | Slowest recovery after heal |
| --- | --- | ---: | ---: | --- | --- | --- | --- | --- | --- | --- | --- | ---: | ---: | ---: |
| `instance-crash` | F1 | 5 | 4 | pass | pass | pass | pass | pass | pass | pass | **FAIL** (1) | 120396 | 385 | 61.0 s |
| `instance-down-long` | F1, held | 5 | 5 | pass | pass | pass | pass | pass | pass | pass | pass | 80510 | 240 | 9.8 s |
| `instance-pause` | F2 | 5 | 0 | unknown | unknown | unknown | pass | **FAIL** (5) | unknown | unknown | unknown | 72136 | 154 | 26.9 s |
| `instance-replace` | F3 | 5 | 5 | pass | pass | pass | pass | pass | pass | pass | pass | 206289 | 221 | 8.7 s |
| `node-crash` | F4 | 5 | 5 | pass | pass | pass | pass | pass | pass | pass | pass | 115999 | 289 | 16.5 s |
| `node-isolation` | F5 | 5 | 0 | pass | pass | pass | pass | pass | **FAIL** (5) | pass | **FAIL** (2) | 119406 | 285 | 57.8 s |
| `instance-isolation` | F6 | 5 | 1 | pass | pass | pass | pass | **FAIL** (1) | pass | **FAIL** (1) | **FAIL** (3) | 86255 | 571 | 302.5 s |
| `bucket-outage` | F7 | 5 | 0 | unknown | unknown | unknown | pass | **FAIL** (5) | pass | unknown | unknown | 46742 | 0 | 0.0 s |
| `kill-in-step` | F8 | 5 | 5 | pass | pass | pass | pass | pass | pass | pass | pass | 71822 | 599 | 16.5 s |

| Run | Seed | I1 | I2 | I3 | I4 | I5 | I6 | I7 | I8 | Acknowledged appends | Actors interrupted | Slowest recovery after heal |
| --- | ---: | --- | --- | --- | --- | --- | --- | --- | --- | ---: | ---: | ---: |
| `instance-crash-s1-1793826176` | 1 | pass | pass | pass | pass | pass | pass | pass | pass | 24365 | 92 | 6.2 s |
| `instance-crash-s2-1793826661` | 2 | pass | pass | pass | pass | pass | pass | pass | pass | 25291 | 39 | 2.2 s |
| `instance-crash-s3-1793827143` | 3 | pass | pass | pass | pass | pass | pass | pass | pass | 24173 | 92 | 5.7 s |
| `instance-crash-s4-1793827631` | 4 | pass | pass | pass | pass | pass | pass | pass | pass | 23892 | 81 | 8.2 s |
| `instance-crash-s5-1793828116` | 5 | pass | pass | pass | pass | pass | pass | pass | **FAIL** | 22675 | 81 | 61.0 s |
| `instance-down-long-s1-1793828581` | 1 | pass | pass | pass | pass | pass | pass | pass | pass | 11993 | 59 | 9.8 s |
| `instance-down-long-s2-1793829071` | 2 | pass | pass | pass | pass | pass | pass | pass | pass | 28059 | 20 | 0.8 s |
| `instance-down-long-s3-1793829520` | 3 | pass | pass | pass | pass | pass | pass | pass | pass | 12244 | 50 | 6.4 s |
| `instance-down-long-s4-1793830011` | 4 | pass | pass | pass | pass | pass | pass | pass | pass | 13204 | 60 | 6.8 s |
| `instance-down-long-s5-1793830494` | 5 | pass | pass | pass | pass | pass | pass | pass | pass | 15010 | 51 | 1.8 s |
| `instance-pause-s1-1793830934` | 1 | unknown | unknown | unknown | pass | **FAIL** | pass | unknown | unknown | 14544 | 59 | 4.0 s |
| `instance-pause-s2-1793831736` | 2 | unknown | unknown | unknown | pass | **FAIL** | unknown | unknown | unknown | 29247 | 28 | -11.5 s |
| `instance-pause-s3-1793832173` | 3 | unknown | unknown | unknown | pass | **FAIL** | pass | unknown | unknown | 6687 | 27 | 26.9 s |
| `instance-pause-s4-1793832968` | 4 | unknown | unknown | unknown | pass | **FAIL** | pass | unknown | unknown | 6772 | 13 | 15.8 s |
| `instance-pause-s5-1793833771` | 5 | unknown | unknown | unknown | pass | **FAIL** | pass | unknown | unknown | 14886 | 27 | 0.1 s |
| `instance-replace-s1-1793834576` | 1 | pass | pass | pass | pass | pass | pass | pass | pass | 40891 | 58 | 8.7 s |
| `instance-replace-s2-1793835180` | 2 | pass | pass | pass | pass | pass | pass | pass | pass | 41825 | 27 | 1.8 s |
| `instance-replace-s3-1793835797` | 3 | pass | pass | pass | pass | pass | pass | pass | pass | 40999 | 57 | 6.5 s |
| `instance-replace-s4-1793836415` | 4 | pass | pass | pass | pass | pass | pass | pass | pass | 41120 | 53 | 5.5 s |
| `instance-replace-s5-1793837026` | 5 | pass | pass | pass | pass | pass | pass | pass | pass | 41454 | 26 | 5.1 s |
| `node-crash-s1-1793837645` | 1 | pass | pass | pass | pass | pass | pass | pass | pass | 23239 | 66 | 15.7 s |
| `node-crash-s2-1793838139` | 2 | pass | pass | pass | pass | pass | pass | pass | pass | 23372 | 48 | 14.5 s |
| `node-crash-s3-1793838626` | 3 | pass | pass | pass | pass | pass | pass | pass | pass | 23134 | 55 | 14.6 s |
| `node-crash-s4-1793839111` | 4 | pass | pass | pass | pass | pass | pass | pass | pass | 23091 | 67 | 16.5 s |
| `node-crash-s5-1793839561` | 5 | pass | pass | pass | pass | pass | pass | pass | pass | 23163 | 53 | 14.6 s |
| `node-isolation-s1-1793840012` | 1 | pass | pass | pass | pass | pass | **FAIL** | pass | **FAIL** | 20234 | 95 | -1.6 s |
| `node-isolation-s2-1793840509` | 2 | pass | pass | pass | pass | pass | **FAIL** | pass | pass | 26189 | 31 | 0.0 s |
| `node-isolation-s3-1793841006` | 3 | pass | pass | pass | pass | pass | **FAIL** | pass | pass | 26470 | 33 | -2.2 s |
| `node-isolation-s4-1793841458` | 4 | pass | pass | pass | pass | pass | **FAIL** | pass | **FAIL** | 20165 | 99 | 57.8 s |
| `node-isolation-s5-1793841921` | 5 | pass | pass | pass | pass | pass | **FAIL** | pass | pass | 26348 | 27 | -0.5 s |
| `instance-isolation-s1-1793842420` | 1 | pass | pass | pass | pass | pass | pass | pass | **FAIL** | 16112 | 117 | 30.8 s |
| `instance-isolation-s2-1793842922` | 2 | pass | pass | pass | pass | pass | pass | **FAIL** | pass | 19134 | 113 | 4.8 s |
| `instance-isolation-s3-1793843421` | 3 | pass | pass | pass | pass | **FAIL** | pass | pass | **FAIL** | 16629 | 113 | 302.5 s |
| `instance-isolation-s4-1793844127` | 4 | pass | pass | pass | pass | pass | pass | pass | **FAIL** | 16493 | 114 | 60.7 s |
| `instance-isolation-s5-1793844626` | 5 | pass | pass | pass | pass | pass | pass | pass | pass | 17887 | 114 | 5.8 s |
| `bucket-outage-s1-1793845126` | 1 | unknown | unknown | unknown | pass | **FAIL** | pass | unknown | unknown | 9348 | 0 | 0.0 s |
| `bucket-outage-s2-1793845903` | 2 | unknown | unknown | unknown | pass | **FAIL** | pass | unknown | unknown | 9412 | 0 | 0.0 s |
| `bucket-outage-s3-1793846717` | 3 | unknown | unknown | unknown | pass | **FAIL** | pass | unknown | unknown | 9244 | 0 | 0.0 s |
| `bucket-outage-s4-1793847542` | 4 | unknown | unknown | unknown | pass | **FAIL** | pass | unknown | unknown | 9351 | 0 | 0.0 s |
| `bucket-outage-s5-1793848359` | 5 | unknown | unknown | unknown | pass | **FAIL** | pass | unknown | unknown | 9387 | 0 | 0.0 s |
| `kill-in-step-s1-1793849176` | 1 | pass | pass | pass | pass | pass | pass | pass | pass | 14595 | 113 | 14.5 s |
| `kill-in-step-s2-1793849634` | 2 | pass | pass | pass | pass | pass | pass | pass | pass | 13795 | 132 | 16.5 s |
| `kill-in-step-s3-1793850091` | 3 | pass | pass | pass | pass | pass | pass | pass | pass | 14400 | 108 | 15.7 s |
| `kill-in-step-s4-1793850549` | 4 | pass | pass | pass | pass | pass | pass | pass | pass | 14566 | 126 | 15.3 s |
| `kill-in-step-s5-1793851042` | 5 | pass | pass | pass | pass | pass | pass | pass | pass | 14466 | 120 | 14.7 s |
