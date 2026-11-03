#!/usr/bin/env bash
# Runs one engine, one node, the sink and the mock LLM as host processes, with no
# Docker needed. All state lives in .run/, which `down` leaves for inspection and
# `up` clears.
#
#   cluster/local.sh up | down | endpoint
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
run="$root/.run"
token_file="$run/token"
STOP_GRACE_S="${STOP_GRACE_S:-30}"

start() { # name, command...
	local name="$1"
	shift
	# Its own process group, so `down` also stops children such as pnpm's node.
	setsid "$@" >"$run/logs/$name.log" 2>&1 &
	echo $! >"$run/$name.pid"
	echo "local: started $name (pid $!)"
}

wait_for() { # name, url
	local deadline=$((SECONDS + 30))
	until curl -fsS -m 2 "$2" >/dev/null 2>&1; do
		if ((SECONDS > deadline)); then
			echo "local: $1 did not answer at $2, see $run/logs/$1.log" >&2
			exit 1
		fi
		sleep 0.2
	done
}

stop() { # name
	local pid_file="$run/$1.pid" group
	[[ -f "$pid_file" ]] || return 0
	group="$(cat "$pid_file")"
	kill -- "-$group" 2>/dev/null || true
	for _ in $(seq 1 "$((STOP_GRACE_S * 2))"); do
		kill -0 -- "-$group" 2>/dev/null || break
		sleep 0.5
	done
	if kill -9 -- "-$group" 2>/dev/null; then
		echo "local: $1 ignored SIGTERM for ${STOP_GRACE_S}s, killed it" >&2
	fi
	rm -f "$pid_file"
}

# Nodes go first: they drain through the engine, so stopping it first strands them.
down() {
	for name in node-b node-a mock-llm effects-sink engine; do
		stop "$name"
	done
}

up() {
	down
	# A leftover engine would pass the health checks with the wrong token.
	if curl -fsS -m 2 http://localhost:6420/health >/dev/null 2>&1; then
		echo "local: something else is already serving port 6420" >&2
		exit 1
	fi
	rm -rf "$run"
	mkdir -p "$run/logs" "$run/log" "$run/objects"
	openssl rand -hex 32 >"$token_file"
	local token
	token="$(cat "$token_file")"

	cat >"$run/engine.json" <<EOF
{
	"storage": {
		"log_directory": "$run/log",
		"auto_provision_partitions": 1,
		"balancing": { "enabled": false }
	},
	"object_storage": {
		"primary": { "file_system": { "path": "$run/objects" } }
	}
}
EOF

	cargo build --quiet --manifest-path "$root/Cargo.toml" -p effects-sink -p mock-llm -p chaos-driver

	local engine
	engine="$(find "$root/node_modules/.pnpm" -type f -name rivet-engine | head -1)"
	[[ -x "$engine" ]] || {
		echo "local: no rivet-engine binary found, run pnpm install" >&2
		exit 1
	}

	RIVET__AUTH__ADMIN_TOKEN="$token" start engine "$engine" --config "$run/engine.json"
	RIVET_TOKEN="$token" "$root/cluster/bootstrap.sh"

	SINK_DB="$run/effects.db" start effects-sink "$root/target/debug/effects-sink"
	wait_for effects-sink http://localhost:6510/health
	start mock-llm "$root/target/debug/mock-llm"
	wait_for mock-llm http://localhost:6511/health

	(
		cd "$root/workload"
		NODE_ID=node-a RIVET_POOL=default RIVET_ENDPOINT="http://default:$token@localhost:6420" \
			start node-a pnpm run start
	)
	echo "local: up"
}

case "${1:-}" in
up) up ;;
down) down ;;
endpoint) echo "http://default:$(cat "$token_file")@localhost:6420" ;;
*)
	echo "usage: $0 up|down|endpoint" >&2
	exit 2
	;;
esac
