#!/usr/bin/env bash
# Starts the three-instance cluster from scratch and creates the default pool.
#
#   cluster/up.sh      # build, start, wait, create the pool
#   cluster/down.sh    # stop it and delete its volumes
set -euo pipefail

cluster="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
env_file="$cluster/.env"

# A fresh admin token for every cluster. Request URLs carry it, so it ends up in
# archived logs and reports; by then the cluster it opened is gone.
echo "RIVET_TOKEN=$(openssl rand -hex 32)" >"$env_file"
# shellcheck source=/dev/null
source "$env_file"

compose() { docker compose --project-directory "$cluster" -f "$cluster/compose.yaml" "$@"; }

compose build --quiet
# Rivet's VM guide: start every instance within a minute of the others, so the
# first storage placement uses all of them. `up` starts them together.
compose up --detach --quiet-pull s3 engine-1 engine-2 engine-3 haproxy effects-sink mock-llm
RIVET_TOKEN="$RIVET_TOKEN" \
	ENGINES="http://localhost:7421 http://localhost:7422 http://localhost:7423" \
	WAIT_S="${WAIT_S:-300}" \
	"$cluster/bootstrap.sh"
# The control plane refuses a node whose pool doesn't exist yet.
compose up --detach node-a node-b
