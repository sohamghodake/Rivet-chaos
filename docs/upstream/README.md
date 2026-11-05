# Upstream drafts

Issues, comments and PRs for Rivet, written and ready to paste. None has been sent. Before sending any of them: retest on the latest release, search the issues again, and push this repo so its links work.

| Draft | Kind | Status |
| --- | --- | --- |
| [early-placement.md](early-placement.md) | New issue | Ready. A fresh cluster can bootstrap every tablet on one instance; losing that disk leaves the whole cluster unavailable. Reproduced on 3.0.0 and on 3.0.5, which has the maintainer's zone fix. The [Discord reply](../discord-reply.md) asks whether to file it |
| [lost-alarms.md](lost-alarms.md) | New issue, referencing #5833 | Ready. Reproduced with `repro/lost-alarms` on 3.0.0 (29 of 13,671 acknowledged bookings never ran) and 3.0.5 (13 of 14,089). Framed as a question about intended semantics |
| [s3-http-endpoint.md](s3-http-endpoint.md) | New issue, then a one-line PR | Ready. Confirmed on main (ac94186) |
| [client-readme.md](client-readme.md) | PR | Compile the corrected example first |
| [slatedb-wall-clock.md](slatedb-wall-clock.md) | New issue | Needs a repro on an ordinary Linux host |
| #5957 comment | Comment on an open issue | Not written yet. It didn't reproduce on settled clusters (0 of 5 `instance-down-long` runs, 1 scripted attempt); the first attempt hit the early-placement problem instead |

Dropped: the self-hosting docs' rejected config keys were fixed on 3 November in rivet-dev/website commit 6417f2d (#173).
