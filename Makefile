HARNESS = target/release/rivet-chaos
SEEDS ?= 1 2 3 4 5 6 7 8 9 10
PHASE3_SEEDS ?= 1 2 3 4 5
PHASE3_SCENARIOS ?= instance-crash instance-down-long instance-pause instance-replace node-crash \
	node-isolation instance-isolation bucket-outage kill-in-step

.PHONY: test harness up down run report summary dashboard-data dashboard phase1 phase2 phase3 local-up local-down

test:
	cargo test --workspace
	pnpm --dir workload run check-types
	pnpm --dir dashboard install --frozen-lockfile --silent
	pnpm --dir dashboard run check-types

harness:
	cargo build --release -p chaos-cli

# The three-instance cluster, for poking at by hand.
up:
	cluster/up.sh

down:
	cluster/down.sh

# make run SCENARIO=scenarios/instance-crash.toml [SEED=7]
run: harness
	$(HARNESS) run $(SCENARIO) $(if $(SEED),--seed $(SEED))

# make report RUN=results/<run>
report: harness
	$(HARNESS) report $(RUN)

# make summary [RESULTS=results/phase3]: one table of every run in a folder.
summary: harness
	$(HARNESS) summary $(or $(RESULTS),results/phase3)

# The dashboard's data, from every archived run.
dashboard-data: harness
	$(HARNESS) export results/phase2 results/phase3 results/probes --out dashboard/public/data

# The dashboard as a static site in dashboard/dist; `pnpm --dir dashboard dev` serves it live.
dashboard: dashboard-data
	pnpm --dir dashboard install --frozen-lockfile
	pnpm --dir dashboard run build

# Phase 2's exit criteria: instance-crash once per seed, carrying on past failures.
phase2: harness
	@failed=""; for seed in $(SEEDS); do \
		$(HARNESS) run scenarios/instance-crash.toml --seed $$seed --results results/phase2 || failed="$$failed $$seed"; \
	done; \
	if [ -n "$$failed" ]; then echo "phase2: seeds that did not pass:$$failed"; exit 1; fi; \
	echo "phase2: every seed passed"

# Phase 3: every fault in the catalogue once per seed, carrying on past failures,
# then one table of the lot in results/phase3/summary.md.
phase3: harness
	@failed=""; for scenario in $(PHASE3_SCENARIOS); do for seed in $(PHASE3_SEEDS); do \
		$(HARNESS) run scenarios/$$scenario.toml --seed $$seed --results results/phase3 || failed="$$failed $$scenario/$$seed"; \
	done; done; \
	$(HARNESS) summary results/phase3 > results/phase3/summary.md; \
	if [ -n "$$failed" ]; then echo "phase3: runs that did not pass:$$failed"; exit 1; fi; \
	echo "phase3: every run passed"

# Phase 1's check, on one engine binary from npm, without Docker.
local-up:
	cluster/local.sh up

local-down:
	cluster/local.sh down

phase1:
	cluster/local.sh up
	cargo build --quiet -p chaos-driver
	RIVET_ENDPOINT="$$(cluster/local.sh endpoint)" target/debug/phase1-check; \
		status=$$?; cluster/local.sh down; exit $$status
