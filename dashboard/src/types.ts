// The JSON `rivet-chaos export` writes: `index.json` and `runs/<run>.json`.

export type Status = "pass" | "fail" | "unknown";

export interface IndexEntry {
	run: string;
	/** The folder the run is in, such as `phase2` or `phase3`. */
	group: string;
	scenario: string;
	seed: number;
	started_wall_ms: number;
	verdict: Status;
	statuses: Record<string, Status>;
	acknowledged: number;
	faults: number;
	interrupted: number;
	slowest_after_heal_s: number;
}

export interface Fault {
	fault: string;
	target: string;
	network: string | null;
	start_s: number;
	heal_s: number | null;
}

/** Ledger appends invoked during one second of the run. */
export interface Second {
	t: number;
	acknowledged: number;
	unknown: number;
	failed: number;
	p50_ms: number | null;
	p99_ms: number | null;
	max_ms: number | null;
}

export interface Recovery {
	fault: number;
	actor: string;
	failed_s: number;
	answered_s: number;
	outage_s: number;
	after_heal_s: number;
}

export interface Invariant {
	id: string;
	name: string;
	status: Status;
	checked: number;
	violation_count: number;
	violations: string[];
	notes: string[];
}

export interface Run {
	run: string;
	group: string;
	scenario: string;
	seed: number;
	started_wall_ms: number;
	duration_s: number;
	verdict: Status;
	versions: Record<string, string>;
	host: Record<string, string>;
	invariants: Invariant[];
	faults: Fault[];
	seconds: Second[];
	recoveries: Recovery[];
	totals: {
		appends: number;
		acknowledged: number;
		unknown: number;
		failed: number;
		reads: number;
		turns_sent: number;
		turns_acknowledged: number;
	};
}
