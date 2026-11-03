HARNESS = target/release/rivet-chaos
SEEDS ?= 1 2 3 4 5 6 7 8 9 10

.PHONY: test harness up down run report phase1 phase2 local-up local-down

test:
	cargo test --workspace
	pnpm --dir workload run check-types

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

# Phase 2's exit criteria: instance-crash once per seed, carrying on past failures.
phase2: harness
	@failed=""; for seed in $(SEEDS); do \
		$(HARNESS) run scenarios/instance-crash.toml --seed $$seed || failed="$$failed $$seed"; \
	done; \
	if [ -n "$$failed" ]; then echo "phase2: seeds that did not pass:$$failed"; exit 1; fi; \
	echo "phase2: every seed passed"

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
