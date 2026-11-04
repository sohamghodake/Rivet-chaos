#!/usr/bin/env python3
"""A fresh three-instance cluster can place every storage tablet on one instance.

About a minute after the engines start, the storage coordinator places the
cluster's 32 tablets on the workers it has admitted so far. Even when all three
engines start within a second, it sometimes has admitted only one, and puts every
tablet there with a single replica; the others follow a minute or so later. Until
then that instance holds the only copy of everything.

This starts fresh clusters until one places its tablets on a single instance,
writes to some ledgers, and then destroys that instance and its log volume (as
F3 does) before the copies are made. A fresh instance takes its place, and every
ledger is read back to see whether the acknowledged writes survived.

Needs Docker with Compose, curl and openssl, and this repo's cluster. Run from
anywhere:

    repro/early-placement/repro.py

Exits 0 if the single-instance placement was caught and the instance destroyed,
whatever the reads showed; the summary says what was lost.
"""

import json
import os
import re
import secrets
import subprocess
import sys
import time
import urllib.error
import urllib.request
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
CLUSTER = REPO / "cluster"
ENGINES = ["engine-1", "engine-2", "engine-3"]
METRICS = {"engine-1": 7431, "engine-2": 7432, "engine-3": 7433}
ATTEMPTS = int(os.environ.get("ATTEMPTS", "8"))
LEDGERS = int(os.environ.get("LEDGERS", "20"))
WRITES = int(os.environ.get("WRITES", "5"))

started = time.monotonic()


def log(message):
    print(f"{time.strftime('%H:%M:%S', time.gmtime())}  {message}", flush=True)


def compose(*args, check=True, capture=False):
    command = ["docker", "compose", "--project-directory", str(CLUSTER), "-f", str(CLUSTER / "compose.yaml"), *args]
    result = subprocess.run(command, check=check, text=True, capture_output=capture)
    return result.stdout.strip() if capture else None


def container(service):
    return compose("ps", "--all", "-q", service, capture=True)


def metrics(engine):
    try:
        with urllib.request.urlopen(f"http://localhost:{METRICS[engine]}/metrics", timeout=2) as response:
            return response.read().decode()
    except OSError:
        return ""


def placement():
    """The coordinator's placement counts, from whichever engine runs it."""
    for engine in ENGINES:
        counts = dict(re.findall(r'^rivet_depot_coordinator_placement\{kind="(\w+)"\} (\d+)', metrics(engine), re.M))
        if counts:
            return {kind: int(value) for kind, value in counts.items()}
    return {}


def maintainer(engine):
    """True if the engine maintains at least one tablet."""
    return "rivet_depot_busiest_tablet_info{" in metrics(engine)


def call(token, key, action, args=()):
    """One ledger action through the load balancer. Returns (status, output)."""
    url = (
        f"http://localhost:6420/gateway/ledger/action/{action}?rvt-namespace=default&rvt-method=getOrCreate"
        f"&rvt-key={key}&rvt-runner=default&rvt-crash-policy=sleep&rvt-token={token}"
    )
    request = urllib.request.Request(
        url,
        data=json.dumps({"args": list(args)}).encode(),
        headers={"content-type": "application/json", "x-rivet-encoding": "json"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(request, timeout=5) as response:
            return response.status, json.loads(response.read()).get("output")
    except urllib.error.HTTPError as error:
        return error.code, None
    except OSError:
        return 0, None


def start_cluster():
    """A fresh cluster, as cluster/up.sh starts it but without waiting for storage to settle."""
    subprocess.run([str(CLUSTER / "down.sh")], check=False, capture_output=True)
    token = secrets.token_hex(32)
    (CLUSTER / ".env").write_text(f"RIVET_TOKEN={token}\n")
    os.environ["RIVET_TOKEN"] = token
    compose("build", "--quiet")
    compose("up", "--detach", "--quiet-pull", "s3", *ENGINES, "haproxy", "effects-sink", "mock-llm")
    engines_started = time.monotonic()
    env = dict(os.environ, ENGINES=" ".join(f"http://localhost:{port - 10}" for port in METRICS.values()), WAIT_S="300")
    subprocess.run([str(CLUSTER / "bootstrap.sh")], check=True, env=env, capture_output=True)
    compose("up", "--detach", "node-a", "node-b")
    return token, engines_started


def first_placement(engines_started):
    """Waits for the coordinator's first placement and returns its counts."""
    while time.monotonic() - engines_started < 300:
        counts = placement()
        if counts.get("projected", 0) > 0:
            return counts
        time.sleep(0.5)
    raise SystemExit("no placement within 300 s of the engines starting")


def write(token, key):
    """Appends WRITES entries to one ledger in order; returns the acknowledged op_ids."""
    acknowledged = []
    deadline = time.monotonic() + 60
    index = 0
    while index < WRITES and time.monotonic() < deadline:
        op_id = f"w-{key}-{index}"
        status, output = call(token, key, "append", [op_id, index])
        if status == 200 and output in ("inserted", "duplicate"):
            acknowledged.append(op_id)
            index += 1
        elif status == 200:
            raise SystemExit(f"unexpected append result {output!r}")
    return acknowledged


def read_back(token, key, deadline):
    while time.monotonic() < deadline:
        status, output = call(token, key, "read")
        if status == 200:
            return output
        time.sleep(1)
    return None


def main():
    for attempt in range(1, ATTEMPTS + 1):
        log(f"attempt {attempt}: starting a fresh cluster")
        token, engines_started = start_cluster()
        counts = first_placement(engines_started)
        holders = [engine for engine in ENGINES if maintainer(engine)]
        log(
            f"first placement {time.monotonic() - engines_started:.0f} s after the engines started: "
            f"{counts.get('admitted_workers')} instances admitted, {counts.get('projected')} replicas placed, "
            f"maintainers {holders}"
        )
        if counts.get("admitted_workers") == 1 and len(holders) == 1:
            break
    else:
        log(f"no single-instance placement in {ATTEMPTS} attempts")
        return 1
    holder = holders[0]

    log(f"writing {WRITES} entries to each of {LEDGERS} ledgers")
    with ThreadPoolExecutor(max_workers=LEDGERS) as pool:
        acknowledged = dict(zip(range(LEDGERS), pool.map(lambda key: write(token, key), range(LEDGERS))))
    total = sum(len(ops) for ops in acknowledged.values())
    counts = placement()
    learners = re.findall(r'^rivet_depot_learner_phases_total\{result="(\w+)"\} (\d+)', metrics(holder), re.M)
    log(
        f"{total} appends acknowledged; placement now {counts.get('admitted_workers')} instances admitted, "
        f"{counts.get('projected')} of {counts.get('desired')} replicas placed; learner phases on {holder}: {learners or 'none ended'}"
    )

    log(f"destroying {holder} and its log volume")
    volumes = subprocess.run(
        ["docker", "inspect", "-f", '{{range .Mounts}}{{if eq .Type "volume"}}{{.Name}} {{end}}{{end}}', container(holder)],
        check=True, text=True, capture_output=True,
    ).stdout.split()
    subprocess.run(["docker", "rm", "--force", "--volumes", container(holder)], check=True, capture_output=True)
    for volume in volumes:
        subprocess.run(["docker", "volume", "rm", "--force", volume], check=True, capture_output=True)
    destroyed = time.monotonic()
    time.sleep(30)
    log(f"starting a fresh {holder}")
    compose("up", "--detach", "--no-deps", holder)

    log("reading every ledger back, for up to 5 minutes")
    deadline = time.monotonic() + 300
    lost, unreadable = {}, []
    for key in range(LEDGERS):
        found = read_back(token, key, deadline)
        if found is None:
            unreadable.append(key)
            continue
        missing = [op for op in acknowledged[key] if op not in found]
        if missing:
            lost[key] = missing
    log(f"reads finished {time.monotonic() - destroyed:.0f} s after {holder} was destroyed")

    print()
    print(f"First placement: every tablet on {holder} alone.")
    print(f"Acknowledged appends: {total} across {LEDGERS} ledgers.")
    print(f"Ledgers that never answered within 5 minutes: {len(unreadable)} {unreadable or ''}")
    print(f"Acknowledged appends missing from the ledgers that answered: {sum(map(len, lost.values()))}")
    for key, missing in sorted(lost.items()):
        print(f"  ledger {key}: {', '.join(missing)}")
    print("The cluster is left running; cluster/down.sh removes it.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
