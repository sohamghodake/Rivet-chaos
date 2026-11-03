#!/usr/bin/env bash
# Stops the cluster and deletes its volumes, so the next run starts clean. The
# state is thrown away, so nothing gets more than 10 s to shut down.
set -euo pipefail
cluster="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
docker compose --project-directory "$cluster" -f "$cluster/compose.yaml" down --volumes --remove-orphans --timeout 10
