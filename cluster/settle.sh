#!/usr/bin/env bash
# Waits until storage has placed every replica it wants: every instance admitted,
# the coordinator's placement complete, and no replacement pending.
#
# A fresh cluster places its tablets about a minute after start, sometimes on a
# single instance, and copies them to the others over the next minute or so.
# Until then one instance can hold the only copy, so faults are injected only
# once this passes.
#
#   METRICS   the instances' metrics endpoints (default: the three on localhost)
#   WAIT_S    how long to wait (default 600)
set -euo pipefail

METRICS="${METRICS:-http://localhost:7431 http://localhost:7432 http://localhost:7433}"
WAIT_S="${WAIT_S:-600}"
read -ra endpoints <<<"$METRICS"
instances=${#endpoints[@]}

# The coordinator's placement counts, from whichever instance runs it.
placement() {
	local endpoint
	for endpoint in "${endpoints[@]}"; do
		curl -s --max-time 2 "$endpoint/metrics" | grep '^rivet_depot_coordinator_placement{' || true
	done
}

count() { awk -v kind="kind=\"$1\"" 'index($0, kind) { print $2; exit }' <<<"$2"; }

deadline=$((SECONDS + WAIT_S))
last=""
while true; do
	counts="$(placement)"
	admitted="$(count admitted_workers "$counts")"
	desired="$(count desired "$counts")"
	projected="$(count projected "$counts")"
	pending="$(count pending_replacements "$counts")"
	state="admitted ${admitted:-?} of $instances, placed ${projected:-?} of ${desired:-?} replicas, ${pending:-?} replacements pending"
	if [[ "$admitted" == "$instances" && -n "$desired" && "$desired" != 0 && "$projected" == "$desired" && "$pending" == 0 ]]; then
		echo "settle: storage settled: $state"
		exit 0
	fi
	if ((SECONDS >= deadline)); then
		echo "settle: storage didn't settle within ${WAIT_S}s: $state" >&2
		exit 1
	fi
	if [[ "$state" != "$last" ]]; then
		echo "settle: $state"
		last="$state"
	fi
	sleep 2
done
