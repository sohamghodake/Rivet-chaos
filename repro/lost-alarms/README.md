# One-shot schedules lost when an engine is cut off from its peers

Rivet's [schedule docs](https://github.com/rivet-dev/rivet/blob/main/docs/actors/content/docs/schedule.mdx) say schedules "survive sleep, restarts, upgrades, and crashes".

In the harness's F5 and F6 runs, 19 follow-up alarms booked by agent-session workflows never fired. 17 were booked within a second of a partition starting; the other 2 came just after a heal, in a session that then stayed stuck for five minutes. `repro.py` takes the workflow out:

1. 20 `alarm-probe` actors book alarms 2 s ahead from a plain action (`c.schedule.after`), every ~100 ms. An HTTP 200 means the booking was acknowledged.
2. One engine is cut off from the `peer` network for 60 s, twice (engine-1, then engine-3).
3. Each alarm's action posts to the effects sink the moment it runs. The script then lists acknowledged bookings that never reached the sink.

```bash
repro/lost-alarms/repro.py
ENGINE_IMAGE=rivetdev/engine:3.0.5 repro/lost-alarms/repro.py
```

It needs Docker with Compose and Python 3, takes about ten minutes, and exits 0 if any acknowledged booking was lost.

## Result on 3.0.0, 5 November

29 of 13,671 acknowledged bookings never fired (`results/probes/lost-alarms-1793905653/`). All 29 were acknowledged between 1.6 s before and 0.2 s after a partition began, so each came due in the first moments of a partition: 21 around the first, 8 around the second. One actor lost all 12 bookings it made in that window. Every booking acknowledged more than 0.2 s into a partition fired.

## Result on 3.0.5, 5 November

13 of 14,089 acknowledged bookings never fired (`results/probes/lost-alarms-3.0.5-1793906065/`, engine 3.0.5, RivetKit 3.0.0, whose scheduler code is the same as 3.0.5's). The pattern was the same: all were acknowledged 1.6 to 1.7 s before a partition began, 4 around the first and 9 around the second.

## Why

RivetKit claims a due one-shot schedule by deleting it (`claim_one_shots_sql`: `DELETE FROM _rivet_schedule_events WHERE kind = ? AND (event_id, trigger_at) IN …`), commits that, and only then runs the action (`take_due_schedule_dispatches` in `rivetkit-core/src/actor/schedule.rs`).

When the partition starts, SQLite commits in flight from the actors stall for 30 s and then fail with `sqlite commit result is indeterminate`; the rollback then fails with `SQLite database is closed`. The losses fit a claim whose delete did land while its actor closed before dispatching: the event is gone, and its action never ran. We haven't inspected the schedule tables to prove it.

rivet-dev/rivet#5833 describes the same claim-then-dispatch shape for an error returned while the process stays alive, and explicitly sets aside "the crash window inherent to claim-then-dispatch". This is that window, opened by an ordinary partition rather than a process crash, and kept open for the 30 s commit deadline.
