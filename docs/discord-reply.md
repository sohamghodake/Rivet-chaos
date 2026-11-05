# Discord reply draft

The reply to the maintainer's answers, ready to paste. Not sent yet. Commit and push first, so the repo link shows the 3.0.5 retest.

> Thanks, that's really helpful, and Stateright looks fun.
>
> On 4, I retested on 3.0.5, which has your fix (465e5e1). It still happens, on the first fresh cluster I started: about 60 s after start the coordinator bootstrapped all 32 tablets with one worker admitted (`admitted_workers=1 projected_tablets=32`). I then destroyed that instance and its disk and started a fresh one. Every actor stayed unavailable for the 5 minutes I watched: placement stuck at 32/96, `blocked=destination_or_quorum`, `regrants_total{result="no_member"} 32`, and `no live member can take the grant; the tablet stays unavailable until a member returns` for each tablet.
>
> From the code, I think the zone change doesn't reach it when no zones are configured: `placement.rs` still bootstraps "on the destinations that qualify now, even when there are fewer than three", and in my clusters the workers became eligible about 10 s apart even though they started within a second. So the first one sometimes gets everything.
>
> The repro script, logs and metrics are in https://github.com/sohamghodake/Rivet-chaos under `repro/early-placement` and `results/probes/early-placement-3.0.5`. Want me to open an issue with them?
