#!/usr/bin/env python3
"""Does an acknowledged alarm booking survive an engine being cut off from its peers?

In the harness's F5 and F6 runs, every alarm that never fired had been booked
within a second of a partition starting. Those bookings were made inside
workflow steps, though, so a dropped step could explain them as well as a lost
write. This takes the workflow out: 20 `alarm-probe` actors book alarms from a
plain action, and an HTTP 200 means the booking was acknowledged.

It starts a fresh cluster (cluster/up.sh, which waits for storage to settle),
books alarms 2 s out on every actor every ~100 ms, cuts one engine off the peer
network for 60 s, twice, and then checks which bookings fired. An acknowledged
booking that never fires is lost; one whose call timed out or failed may
legitimately be missing.

Needs Docker with Compose and Python 3. Run from anywhere:

    repro/lost-alarms/repro.py

Exits 0 if any acknowledged booking was lost, 1 if none was.
"""

import json
import os
import random
import subprocess
import sys
import threading
import time
import urllib.error
import urllib.request
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
CLUSTER = REPO / "cluster"
ACTORS = int(os.environ.get("ACTORS", "20"))
DELAY_MS = int(os.environ.get("DELAY_MS", "2000"))
PARTITION_S = int(os.environ.get("PARTITION_S", "60"))
# (seconds after booking starts, engine cut off from its peers)
PARTITIONS = [(30, "engine-1"), (120, "engine-3")]
BOOK_FOR_S = 210
SINK = "http://localhost:6510"

started = time.monotonic()


def log(message):
    print(f"{time.strftime('%H:%M:%S', time.gmtime())}  +{time.monotonic() - started:5.1f}s  {message}", flush=True)


def compose(*args, capture=False):
    command = ["docker", "compose", "--project-directory", str(CLUSTER), "-f", str(CLUSTER / "compose.yaml"), *args]
    result = subprocess.run(command, check=True, text=True, capture_output=capture)
    return result.stdout.strip() if capture else None


def book(token, key, booking):
    """One `book` action through the load balancer: 'ok', or 'unknown' if it may or may not have landed."""
    url = (
        f"http://localhost:6420/gateway/alarm-probe/action/book?rvt-namespace=default&rvt-method=getOrCreate"
        f"&rvt-key={key}&rvt-runner=default&rvt-crash-policy=sleep&rvt-token={token}"
    )
    body = json.dumps({"args": [booking, DELAY_MS]}).encode()
    request = urllib.request.Request(url, data=body, headers={"content-type": "application/json", "x-rivet-encoding": "json"}, method="POST")
    try:
        with urllib.request.urlopen(request, timeout=5) as response:
            return "ok" if json.loads(response.read()).get("output") == "booked" else "unknown"
    except (urllib.error.URLError, OSError, ValueError):
        return "unknown"


def partition(engine, seconds, phases):
    container = compose("ps", "-q", engine, capture=True)
    ip = f"10.77.1.1{engine[-1]}"
    phases.append((engine, time.monotonic(), None))
    log(f"cutting {engine} off the peer network for {seconds} s")
    subprocess.run(["docker", "network", "disconnect", "--force", "rivet-chaos_peer", container], check=True)
    time.sleep(seconds)
    subprocess.run(["docker", "network", "connect", "--ip", ip, "--alias", engine, "rivet-chaos_peer", container], check=True)
    phases[-1] = (engine, phases[-1][1], time.monotonic())
    log(f"{engine} reconnected")


def main():
    log("starting a fresh cluster")
    subprocess.run([str(CLUSTER / "down.sh")], check=False, capture_output=True)
    subprocess.run([str(CLUSTER / "up.sh")], check=True)
    token = dict(line.split("=", 1) for line in (CLUSTER / ".env").read_text().split()).get("RIVET_TOKEN", "")

    log(f"warming up {ACTORS} actors")
    for key in range(ACTORS):
        deadline = time.monotonic() + 120
        while book(token, key, f"warm-{key}") != "ok":
            if time.monotonic() > deadline:
                raise SystemExit(f"alarm-probe/{key} never answered")
            time.sleep(0.5)

    bookings = []  # (key, booking, outcome, invoked)
    lock = threading.Lock()
    stop = threading.Event()
    begin = time.monotonic()

    def booker(key):
        count = 0
        while not stop.is_set():
            booking = f"b{count:05d}"
            invoked = time.monotonic()
            outcome = book(token, key, booking)
            with lock:
                bookings.append((key, booking, outcome, invoked))
            count += 1
            time.sleep(random.uniform(0.05, 0.15))

    threads = [threading.Thread(target=booker, args=(key,), daemon=True) for key in range(ACTORS)]
    for thread in threads:
        thread.start()

    phases = []
    for at, engine in PARTITIONS:
        time.sleep(max(0, begin + at - time.monotonic()))
        partition(engine, PARTITION_S, phases)
    time.sleep(max(0, begin + BOOK_FOR_S - time.monotonic()))
    stop.set()
    for thread in threads:
        thread.join()
    log(f"{len(bookings)} bookings made; waiting for alarms to stop arriving")

    fired = set()
    quiet_since = time.monotonic()
    deadline = time.monotonic() + 300
    while time.monotonic() < deadline and time.monotonic() - quiet_since < 30:
        with urllib.request.urlopen(f"{SINK}/alarms?prefix=probe/", timeout=10) as response:
            rows = json.loads(response.read())
        now = {row["key"] for row in rows}
        if len(now) > len(fired):
            quiet_since = time.monotonic()
        fired = now
        time.sleep(5)

    def phase(invoked):
        for engine, start, end in phases:
            if start - 1 <= invoked <= (end or start) + 5:
                return f"around the {engine} partition"
        return "outside partitions"

    summary = {}
    lost = []
    for key, booking, outcome, invoked in bookings:
        was_fired = f"probe/{key}/{booking}" in fired
        row = summary.setdefault((phase(invoked), outcome), [0, 0])
        row[0] += 1
        row[1] += was_fired
        if outcome == "ok" and not was_fired:
            lost.append((round(invoked - begin, 1), key, booking))

    print()
    print(f"{'when':32} {'call result':12} {'bookings':>9} {'fired':>7}")
    for (when, outcome), (count, fired_count) in sorted(summary.items()):
        print(f"{when:32} {outcome:12} {count:9} {fired_count:7}")
    print()
    print(f"Acknowledged bookings that never fired: {len(lost)}")
    for at, key, booking in lost[:30]:
        print(f"  alarm-probe/{key} {booking}, booked {at} s after booking began")
    print("The cluster is left running; cluster/down.sh removes it.")
    return 0 if lost else 1


if __name__ == "__main__":
    sys.exit(main())
