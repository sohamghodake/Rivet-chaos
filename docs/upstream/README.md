# Upstream drafts

Issues, comments and PRs for Rivet, written and ready to paste. None has been sent. Before sending any of them: retest on the latest release, search the issues again, and push this repo so its links work.

| Draft | Kind | Status |
| --- | --- | --- |
| [early-placement.md](early-placement.md) | New issue | Ready. A fresh cluster can bootstrap every tablet on one instance; losing that disk leaves the whole cluster unavailable. Reproduced with `repro/early-placement`. Question 4 of the [Discord draft](../discord-draft.md) asks about it first |
| [s3-http-endpoint.md](s3-http-endpoint.md) | New issue, then a one-line PR | Ready. Confirmed on main (ac94186) |
| [client-readme.md](client-readme.md) | PR | Compile the corrected example first |
| [slatedb-wall-clock.md](slatedb-wall-clock.md) | New issue | Needs a repro on an ordinary Linux host |
| #5957 comment | Comment on an open issue | Waiting for the `instance-down-long` runs and `repro/5957` |

Dropped: the self-hosting docs' rejected config keys were fixed on 3 November in rivet-dev/website commit 6417f2d (#173).
