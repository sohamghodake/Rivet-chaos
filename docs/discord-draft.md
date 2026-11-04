# Discord draft

For Rivet's Discord, ready to paste. Not sent yet.

> Hi! I'm building a chaos-testing harness for Rivet 3.0: three engines in Docker Compose behind HAProxy, a fault injector, and a checker that compares a log of every operation against the documented storage guarantees. Once storage has settled, no acknowledged write has been lost so far. Four questions:
>
> 1. Is there a recommended way to run three instances on one host for testing? I'm using Compose, with `api_peer.advertise_address` set per container on a private network and VersityGW for the bucket. Is there anything that would make results misleading compared with separate VMs?
> 2. Beyond Depot's crash suite and the simulation-only storage settings, is there a team simulation or fault suite that tests or findings like these should plug into?
> 3. Does a `guard/service_unavailable` rejection (`depot_endpoint_unresolved`, `depot_overloaded`) guarantee that the request never reached the actor? None of about 400 such rejections had been applied, and I'd like to treat them as definite failures.
> 4. Is it expected that a fresh cluster's first storage placement sometimes uses one instance, even when all three start within a second? It happened in 4 of 19 clusters. Destroying that instance's disk in the minute before the copies existed left every actor unavailable with `no live member can take the grant`. Happy to file an issue with the repro if it's a bug.

