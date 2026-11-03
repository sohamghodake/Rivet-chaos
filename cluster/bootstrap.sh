#!/usr/bin/env bash
# Waits for /health on every instance, then creates the default pool.
#
#   RIVET_TOKEN=... ENGINES="http://localhost:6420" cluster/bootstrap.sh
#
# A fresh engine can answer /health while its storage still refuses commands, so
# pool creation retries instead of treating the first error as fatal.
set -euo pipefail

: "${RIVET_TOKEN:?RIVET_TOKEN is required}"
ENGINES="${ENGINES:-http://localhost:6420}"
WAIT_S="${WAIT_S:-120}"
deadline=$((SECONDS + WAIT_S))

for engine in $ENGINES; do
	until curl -fsS -m 2 "$engine/health" >/dev/null 2>&1; do
		if ((SECONDS > deadline)); then
			echo "bootstrap: $engine/health not ready after ${WAIT_S}s" >&2
			exit 1
		fi
		sleep 1
	done
	echo "bootstrap: $engine is healthy"
done

first_engine="${ENGINES%% *}"
until response="$(curl -fsS -m 5 -X PUT "$first_engine/pools/default?namespace=default" \
	-H "Authorization: Bearer $RIVET_TOKEN" \
	-H 'Content-Type: application/json' \
	-d '{}' 2>&1)"; do
	if ((SECONDS > deadline)); then
		echo "bootstrap: could not create the pool after ${WAIT_S}s: $response" >&2
		exit 1
	fi
	sleep 1
done
echo "bootstrap: default pool ready"
