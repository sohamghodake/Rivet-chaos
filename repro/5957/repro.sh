#!/usr/bin/env bash
# Reproduces rivet-dev/rivet#5957 on Rivet 3.0.0: after one of three engines is
# SIGKILLed and kept down, the survivors resend an actor stop command to the dead
# engine until it returns, and the actors involved stay unavailable that long.
#
# Needs Docker with Compose, curl and openssl. Uses this repo's three-engine
# cluster (cluster/compose.yaml) and its ledger actor; nothing else.
#
#   repro/5957/repro.sh              # 20 ledgers, the engine stays down 180 s
#   LEDGERS=50 DOWN_S=300 ATTEMPTS=6 repro/5957/repro.sh
#
# It doesn't happen every time, so it tries up to ATTEMPTS (4) fresh clusters.
# Exits 0 once one reproduces: some ledgers answered nothing for the whole
# outage, and the survivors logged stop commands sent to the dead engine.
set -euo pipefail

repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cluster="$repo/cluster"
LEDGERS="${LEDGERS:-20}"
DOWN_S="${DOWN_S:-180}"
ATTEMPTS="${ATTEMPTS:-4}"

compose() { docker compose --project-directory "$cluster" -f "$cluster/compose.yaml" "$@"; }
log() { printf '%s  %s\n' "$(date -u +%H:%M:%S)" "$*"; }

# One ledger action through the load balancer, as rivetkit-client sends it.
# Prints the HTTP status, or 000 if nothing came back within 5 s.
call() {
	local key="$1" action="$2" args="${3:-[]}"
	curl -s --max-time 5 -o /dev/null -w '%{http_code}' -X POST \
		-H 'content-type: application/json' -H 'x-rivet-encoding: json' \
		--data "{\"args\":$args}" \
		"http://localhost:6420/gateway/ledger/action/$action?rvt-namespace=default&rvt-method=getOrCreate&rvt-key=$key&rvt-runner=default&rvt-crash-policy=sleep&rvt-token=$RIVET_TOKEN" ||
		true
}

# Seconds from HAProxy's session age, such as "59s" or "2m3s".
age_seconds() {
	awk '{ n = 0; s = $0; while (match(s, /[0-9]+[dhms]/)) { v = substr(s, RSTART, RLENGTH - 1); u = substr(s, RSTART + RLENGTH - 1, 1); n += v * (u == "d" ? 86400 : u == "h" ? 3600 : u == "m" ? 60 : 1); s = substr(s, RSTART + RLENGTH) } print n }' <<<"$1"
}

# The engine holding a node's envoy WebSocket: the node's oldest session through
# HAProxy, since the envoy connects when the node starts and stays open.
envoy_engine() {
	local node="$1" ip sessions best="" best_age=-1
	ip="$(docker inspect -f '{{(index .NetworkSettings.Networks "rivet-chaos_front").IPAddress}}' "$(compose ps -q "$node")")"
	sessions="$(compose exec -T haproxy sh -c 'echo "show sess" | nc 127.0.0.1 9999' | grep "src=$ip:" || true)"
	while read -r line; do
		[[ -z "$line" ]] && continue
		local engine age
		engine="$(grep -o 'srv=engine-[0-9]' <<<"$line" | cut -d= -f2)"
		age="$(age_seconds "$(grep -o 'age=[0-9dhms]*' <<<"$line" | cut -d= -f2)")"
		if [[ -n "$engine" && "$age" -gt "$best_age" ]]; then
			best="$engine"
			best_age="$age"
		fi
	done <<<"$sessions"
	echo "$best"
}

# up.sh waits until storage has copied every tablet to every instance. Before
# that, one instance can hold the only copy, and killing it takes everything down
# for a different reason (see repro/early-placement).
# One attempt on a fresh cluster. Returns 0 if it reproduced, 1 if not, 2 if the
# cluster or the script failed. It runs as a condition, where `set -e` doesn't
# apply, so each step that must work says so.
attempt() {
	log "starting a fresh cluster"
	"$cluster/down.sh" >/dev/null 2>&1 || true
	"$cluster/up.sh" || return 2
	# shellcheck source=/dev/null
	source "$cluster/.env"
	log "engine image: $(docker inspect -f '{{.Config.Image}}' "$(compose ps -q engine-1)")"

	log "writing to $LEDGERS ledgers until each has answered"
	local deadline=$((SECONDS + 180)) key
	for ((key = 0; key < LEDGERS; key++)); do
		until [[ "$(call "$key" append "[\"seed-$key\", 0]")" == 200 ]]; do
			((SECONDS < deadline)) || { log "ledger $key never answered; the cluster isn't healthy"; return 2; }
			sleep 1
		done
	done

	local node victim peer_ip since killed
	for node in node-a node-b; do
		log "$node's envoy connection is on $(envoy_engine "$node")"
	done
	victim="$(envoy_engine node-a)"
	[[ -n "$victim" ]] || victim="$(envoy_engine node-b)"
	[[ -n "$victim" ]] || { log "couldn't find either node's envoy connection in HAProxy"; return 2; }
	peer_ip="$(docker inspect -f '{{(index .NetworkSettings.Networks "rivet-chaos_peer").IPAddress}}' "$(compose ps -q "$victim")")"

	since="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
	log "SIGKILL $victim ($peer_ip), keeping it down for up to $DOWN_S s"
	docker kill --signal KILL "$(compose ps -q "$victim")" >/dev/null || return 2
	killed=$SECONDS

	# Every ledger is read at once, every 5 s. A ledger counts as answering during
	# the outage if any read succeeds from 10 s after the kill, once the first
	# timeouts have passed. Once every ledger has answered, this attempt can't
	# reproduce, so it ends early.
	local -A answered=()
	local elapsed ok
	while ((SECONDS - killed < DOWN_S)); do
		elapsed=$((SECONDS - killed))
		for ((key = 0; key < LEDGERS; key++)); do
			call "$key" read >"$reads/$key" &
		done
		wait
		ok=0
		for ((key = 0; key < LEDGERS; key++)); do
			if [[ "$(cat "$reads/$key")" == 200 ]]; then
				ok=$((ok + 1))
				((elapsed >= 10)) && answered[$key]=1
			fi
		done
		log "+$elapsed s: $ok of $LEDGERS ledgers answered"
		((${#answered[@]} == LEDGERS)) && break
		sleep 5
	done

	local stuck=() survivors=() engine resends not_ready started
	for ((key = 0; key < LEDGERS; key++)); do
		[[ -n "${answered[$key]:-}" ]] || stuck+=("$key")
	done
	for engine in engine-1 engine-2 engine-3; do
		[[ "$engine" != "$victim" ]] && survivors+=("$engine")
	done
	resends="$(compose logs --no-color --since "$since" "${survivors[@]}" | grep 'failed to send stop command' | grep -c "$peer_ip" || true)"
	not_ready="$(compose logs --no-color --since "$since" "${survivors[@]}" | grep -c 'actor did not become ready before the gateway deadline' || true)"

	log "starting $victim again"
	docker start "$(compose ps --all -q "$victim")" >/dev/null || return 2
	started=$SECONDS
	for key in "${stuck[@]}"; do
		until [[ "$(call "$key" read)" == 200 ]]; do
			((SECONDS - started < 300)) || { log "ledger $key still not answering 300 s after the restart"; break; }
			sleep 1
		done
	done
	((${#stuck[@]} == 0)) || log "every stuck ledger answered again $((SECONDS - started)) s after the restart"

	echo
	echo "Ledgers that answered nothing while $victim was down ($((started - killed)) s): ${#stuck[@]} of $LEDGERS (${stuck[*]:-none})"
	echo "Survivors' 'failed to send stop command' lines addressed to the dead engine ($peer_ip): $resends"
	echo "Survivors' 'actor did not become ready before the gateway deadline' lines: $not_ready"
	echo
	((${#stuck[@]} > 0 && resends > 0))
}

reads="$(mktemp -d)"
trap 'rm -rf "$reads"' EXIT
for ((try = 1; try <= ATTEMPTS; try++)); do
	log "attempt $try of $ATTEMPTS"
	status=0
	attempt || status=$?
	if ((status == 0)); then
		echo "Reproduced rivet-dev/rivet#5957 on attempt $try. The cluster is left running for its logs:"
		echo "  docker compose -p rivet-chaos logs engine-1 engine-2 engine-3   (cluster/down.sh removes it)"
		exit 0
	fi
	((status == 2)) && exit 2
	log "not reproduced on attempt $try"
done
echo "Not reproduced in $ATTEMPTS attempts."
exit 1
