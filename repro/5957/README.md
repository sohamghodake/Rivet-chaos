# rivet-dev/rivet#5957 on Rivet 3.0.0

[#5957](https://github.com/rivet-dev/rivet/issues/5957) reports that after one of three engines is SIGKILLed and kept down, the survivors keep resending an actor's stop command to the dead engine, and the actor stays unavailable until that engine returns. It was reported on 3.0.0-preview.4, on three VMs, in 1 of 4 runs. The reporters guessed it needs the killed engine to hold the node's envoy connection.

`repro.sh` reproduces it on 3.0.0 final, on one machine, by killing the engine that holds a node's envoy connection:

```bash
repro/5957/repro.sh
```

It needs Docker with Compose, `curl` and `openssl`, and takes about six minutes:

1. Start this repo's cluster: three engines (`rivetdev/engine:3.0.0`, pinned by digest) behind HAProxy, a VersityGW bucket, and two RivetKit nodes running the `ledger` actor.
2. Write once to each of 20 ledgers, through the gateway's HTTP API as `rivetkit-client` does.
3. Ask HAProxy which engine holds node-a's envoy WebSocket: the node's oldest session.
4. SIGKILL that engine and keep it down for 180 s, reading every ledger every few seconds.
5. Count the ledgers that answered nothing for the whole outage, and the survivors' `failed to send stop command` lines addressed to the dead engine's peer address.
6. Start the engine again and time how long the stuck ledgers take to answer.

It exits 0 when it reproduces, and leaves the cluster running for its logs; `cluster/down.sh` removes it. `LEDGERS` and `DOWN_S` change the size and the outage.
