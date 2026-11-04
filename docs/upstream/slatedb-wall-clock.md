# Issue draft: SlateDB is given a wall clock that can go backwards

For rivet-dev/rivet. Not filed: it still needs a repro on an ordinary Linux host, because ours comes from a WSL2 machine whose clock steps backwards every ~29 s, and stepping the clock on purpose there would affect every container on the machine.

## Repro still to do

On a disposable Linux VM with one engine (`rivetdev/engine:3.0.0`, `storage.log_directory` set, an S3 bucket):

1. Put some load on it, such as `repro/5957`'s appends.
2. Step the wall clock back: `sudo date -s "-2 seconds"`, a few times, a few seconds apart.
3. Look for `elapsed time is negative` panics in `slatedb-common`, `filesystem apply failed in the store; fencing` with `invalid clock tick, must be monotonic`, and `tablet maintainer fenced`.

---

**Title:** 3.0.0: `SlateClock` returns wall time, so a backward clock step panics SlateDB's background tasks and fences tablets

## What happens

When the host's wall clock steps backwards, as NTP or a hypervisor's time sync can do:

- SlateDB's ticker panics: `panicked at slatedb-common-0.17.0/src/clock.rs:120:14: elapsed time is negative: OutOfRangeError(())`. That kills the database's `compactor`, `compaction_worker` or `l0_manifest_writer` task, and the store fences the tablet (`filesystem flush failed in the store; fencing`, `Closed error: background task panicked`).
- SlateDB's write path rejects the tick: `filesystem apply failed in the store; fencing`, `invalid clock tick, must be monotonic. last_tick=…, next_tick=…`.
- Each fenced tablet is then recovered under a new grant.

Over ten five-minute runs on a host whose clock stepped back about 0.6 s twice every 29 s, the three engines logged 54 panics and 803 `invalid clock tick` fencings. Clients saw almost nothing: no errors outside our injected faults, and 29 acknowledgements over 300 ms (worst 0.87 s). But every step makes storage throw away and rebuild state.

## Cause

`engine/packages/env/src/backends/production/slatedb.rs:711`:

```rust
impl SystemClock for SlateClock {
	fn now(&self) -> DateTime<Utc> {
		let time = self.0.observe().unwrap_or_else(|error| clock_failure(error));
		DateTime::from_timestamp_millis(time.unix_ms)
```

`unix_ms` is `SystemTime::now()` (`production/clock.rs`), so it moves backwards when the wall clock does, even though the engine's own `TokioClock` uses `CLOCK_BOOTTIME` and treats a regression as fatal. SlateDB expects its clock not to go backwards: `SystemClockTicker::calc_duration` calls `.expect("elapsed time is negative")` (unchanged in slatedb-common 0.18.0), and its maintainers describe this exact failure in a review comment on [slatedb/slatedb#2123](https://github.com/slatedb/slatedb/pull/2123). SlateDB's own `DefaultSystemClock` avoids it by anchoring wall time once and adding monotonic elapsed time.

## Suggested fix

Make `SlateClock::now()` monotonic: take `unix_ms` once at construction and add `boot_ms` elapsed since then, as `DefaultSystemClock` does, or never return less than the last value returned.
