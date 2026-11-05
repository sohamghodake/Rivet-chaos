# Discord answers

The questions in [discord-draft.md](discord-draft.md) were posted on Rivet's Discord, and a maintainer answered on 5 November:

| Question | Answer | What we do with it |
| --- | --- | --- |
| 1. Running three instances on one host | Compose is fine; they usually use a local Kubernetes such as k3s, or the one in Docker Desktop | Keep Compose: its per-network disconnects are what make the partition faults precise |
| 2. A team fault suite to plug into | Not yet. They pointed at [Stateright](https://www.stateright.rs/getting-started.html), a Rust model checker with actors | Not now. A model of the storage protocol in Stateright would be a different project from black-box testing |
| 3. Whether a `guard/service_unavailable` rejection means the request never reached the actor | They don't remember | Keep recording them as `info`, the safe choice. Reading the guard's code would settle it |
| 4. A fresh cluster's first placement on one instance | They had just fixed an availability-zone bug that could allow it, on main, and asked for a retest | Retested on 3.0.5, which contains the fix (465e5e163): still happens, on the first fresh cluster, with the same outcome. See [upstream/early-placement.md](upstream/early-placement.md#retest-on-305). Reply drafted in [discord-reply.md](discord-reply.md) |
