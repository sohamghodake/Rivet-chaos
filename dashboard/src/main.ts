import "./style.css";

import { faultLabel, outages, timeline } from "./charts";
import { h } from "./dom";
import { formatCount, formatDate, formatMs, formatMsTick, formatSeconds } from "./format";
import type { IndexEntry, Invariant, Run, Status } from "./types";

const INVARIANTS: [string, string][] = [
	["I1", "Durability"],
	["I2", "No phantom writes"],
	["I3", "No duplicates"],
	["I4", "Real-time visibility"],
	["I5", "Recovery"],
	["I6", "Single activation"],
	["I7", "Workflow effects"],
	["I8", "Alarms"],
];

/** The fault catalogue's order, with the fault each scenario injects. */
const SCENARIOS: [string, string][] = [
	["instance-crash", "F1"],
	["instance-down-long", "F1, held"],
	["instance-pause", "F2"],
	["instance-replace", "F3"],
	["node-crash", "F4"],
	["node-isolation", "F5"],
	["instance-isolation", "F6"],
	["bucket-outage", "F7"],
	["kill-in-step", "F8"],
];

const app = document.querySelector<HTMLElement>("#app");
const runs = new Map<string, Promise<Run>>();
// Kept across renders, so a run's charts stay (dimmed) while the next one loads.
const slots = { filters: h("div"), overview: h("div"), detail: h("div", { class: "run-slot" }) };
let latest = 0;

function groupName(group: string): string {
	const phase = /^phase(\d+)$/.exec(group);
	if (phase) return `Phase ${phase[1]}`;
	return group.charAt(0).toUpperCase() + group.slice(1);
}

function scenarioOrder(scenario: string): number {
	const position = SCENARIOS.findIndex(([name]) => name === scenario);
	return position === -1 ? SCENARIOS.length : position;
}

/** A status with its icon and its word, so it never relies on colour. */
function badge(status: Status | undefined): HTMLElement {
	if (!status) return h("span", { class: "muted" }, "–");
	const [icon, word] = { pass: ["✓", "Pass"], fail: ["✕", "Fail"], unknown: ["?", "Unknown"] }[status];
	return h("span", { class: `badge ${status}` }, h("span", { class: "icon", "aria-hidden": "true" }, icon), word);
}

function tile(label: string, value: string, detail?: string): HTMLElement {
	return h(
		"div",
		{ class: "card tile" },
		h("div", { class: "tile-label" }, label),
		h("div", { class: "tile-value" }, value),
		detail ? h("div", { class: "muted tile-detail" }, detail) : null,
	);
}

function select(label: string, options: [string, string][], value: string, onChange: (value: string) => void): HTMLElement {
	const control = h("select", {}, ...options.map(([key, text]) => h("option", { value: key, selected: key === value }, text)));
	control.addEventListener("change", () => onChange(control.value));
	return h("label", { class: "filter" }, h("span", { class: "muted" }, label), control);
}

interface State {
	group: string;
	run: string | null;
}

function readState(index: IndexEntry[]): State {
	const params = new URLSearchParams(location.hash.slice(1));
	const run = params.get("run");
	const entry = index.find((one) => one.run === run);
	const groups = [...new Set(index.map((one) => one.group))];
	const group = entry?.group ?? params.get("group") ?? groups.find((one) => one === "phase3") ?? groups[0] ?? "";
	return { group, run: entry ? entry.run : null };
}

function writeState(state: State): void {
	const params = new URLSearchParams({ group: state.group });
	if (state.run) params.set("run", state.run);
	location.hash = params.toString();
}

function overview(index: IndexEntry[], state: State): HTMLElement {
	const entries = index.filter((entry) => entry.group === state.group);
	const passed = entries.filter((entry) => entry.verdict === "pass").length;
	const slowest = Math.max(0, ...entries.map((entry) => entry.slowest_after_heal_s));
	const byScenario = new Map<string, IndexEntry[]>();
	for (const entry of [...entries].sort((a, b) => scenarioOrder(a.scenario) - scenarioOrder(b.scenario) || a.seed - b.seed)) {
		byScenario.set(entry.scenario, [...(byScenario.get(entry.scenario) ?? []), entry]);
	}

	const rows = [...byScenario].map(([scenario, group]) => {
		const fault = SCENARIOS.find(([name]) => name === scenario)?.[1] ?? "";
		const cells = INVARIANTS.map(([id]) => {
			const statuses = group.map((entry) => entry.statuses[id]).filter((status): status is Status => !!status);
			if (statuses.length === 0) return h("td", {}, badge(undefined));
			const failed = statuses.filter((status) => status === "fail").length;
			const status: Status = failed > 0 ? "fail" : statuses.includes("unknown") ? "unknown" : "pass";
			return h("td", {}, badge(status), failed > 0 ? h("span", { class: "muted" }, ` ${failed} of ${statuses.length}`) : null);
		});
		const link = h("a", { href: `#${new URLSearchParams({ group: state.group, run: group[0]?.run ?? "" })}` }, scenario);
		return h(
			"tr",
			{},
			h("th", { scope: "row" }, link),
			h("td", {}, fault),
			h("td", { class: "num" }, `${group.filter((entry) => entry.verdict === "pass").length} of ${group.length}`),
			...cells,
			h("td", { class: "num" }, formatCount(group.reduce((sum, entry) => sum + entry.acknowledged, 0))),
			h("td", { class: "num" }, formatSeconds(Math.max(0, ...group.map((entry) => entry.slowest_after_heal_s)))),
		);
	});

	return h(
		"section",
		{ class: "section" },
		h("h2", {}, /^phase\d+$/.test(state.group) ? `${groupName(state.group)} runs` : groupName(state.group)),
		h(
			"div",
			{ class: "tiles" },
			tile("Runs", formatCount(entries.length)),
			tile("Passed every invariant", `${passed} of ${entries.length}`),
			tile("Acknowledged appends", formatCount(entries.reduce((sum, entry) => sum + entry.acknowledged, 0))),
			tile("Slowest recovery after a heal", formatSeconds(slowest), "Across every interrupted actor"),
		),
		h(
			"div",
			{ class: "card table-card" },
			h(
				"table",
				{},
				h(
					"thead",
					{},
					h(
						"tr",
						{},
						h("th", { scope: "col" }, "Scenario"),
						h("th", { scope: "col" }, "Fault"),
						h("th", { scope: "col", class: "num" }, "Passed"),
						...INVARIANTS.map(([id, name]) => h("th", { scope: "col", title: name }, id)),
						h("th", { scope: "col", class: "num" }, "Acknowledged appends"),
						h("th", { scope: "col", class: "num" }, "Slowest recovery"),
					),
				),
				h("tbody", {}, ...rows),
			),
		),
	);
}

function invariantsTable(invariants: Invariant[]): HTMLElement {
	const rows = invariants.map((invariant) =>
		h(
			"tr",
			{},
			h("th", { scope: "row" }, invariant.id),
			h("td", {}, invariant.name),
			h("td", {}, badge(invariant.status)),
			h("td", { class: "num" }, formatCount(invariant.checked)),
			h("td", { class: "num" }, formatCount(invariant.violation_count)),
		),
	);
	const details = invariants.flatMap((invariant) => [
		...invariant.notes.map((note) => h("li", {}, h("strong", {}, `${invariant.id}: `), note)),
		...invariant.violations.map((violation) => h("li", { class: "violation" }, h("strong", {}, `${invariant.id} violation: `), violation)),
		invariant.violation_count > invariant.violations.length
			? h("li", { class: "muted" }, `${invariant.id}: ${invariant.violation_count - invariant.violations.length} more in the run's report`)
			: null,
	]);
	return h(
		"div",
		{ class: "card table-card" },
		h("h3", {}, "Invariants"),
		h(
			"table",
			{},
			h(
				"thead",
				{},
				h(
					"tr",
					{},
					h("th", { scope: "col" }, "ID"),
					h("th", { scope: "col" }, "Invariant"),
					h("th", { scope: "col" }, "Result"),
					h("th", { scope: "col", class: "num" }, "Checked"),
					h("th", { scope: "col", class: "num" }, "Violations"),
				),
			),
			h("tbody", {}, ...rows),
		),
		details.some((item) => item) ? h("ul", { class: "details" }, ...details) : null,
	);
}

/** `Engine image rivetdev/engine:3.0.0@sha256:5ae3c24275ee · rivetkit 3.0.0 · …`, without the report's Markdown. */
function versions(versions: Record<string, string>): string {
	return Object.entries(versions)
		.map(([name, value]) => {
			const plain = value.replaceAll("`", "").replace(/(sha256:[0-9a-f]{12})[0-9a-f]+/, "$1");
			return `${name} ${plain}`;
		})
		.join(" · ");
}

function runView(run: Run): HTMLElement {
	const seconds = run.seconds;
	const unknown = run.totals.unknown + run.totals.failed;
	const slowest = Math.max(0, ...run.recoveries.map((recovery) => recovery.after_heal_s));
	const faults =
		run.faults.length === 0
			? "No faults."
			: run.faults
					.map((fault, index) => `${index + 1}. ${faultLabel(fault)} at ${formatSeconds(fault.start_s)}${fault.heal_s === null ? ", never healed" : `, healed at ${formatSeconds(fault.heal_s)}`}`)
					.join(" · ");

	return h(
		"section",
		{ class: "section" },
		h("div", { class: "run-head" }, h("h2", {}, `${run.scenario}, seed ${run.seed}`), badge(run.verdict)),
		h("p", { class: "muted" }, `${run.run} · started ${formatDate(run.started_wall_ms)}`),
		h("p", { class: "muted" }, versions(run.versions)),
		h("p", { class: "muted" }, faults),
		h(
			"div",
			{ class: "tiles" },
			tile("Acknowledged appends", formatCount(run.totals.acknowledged), `of ${formatCount(run.totals.appends)} invoked`),
			tile("Unknown or failed appends", formatCount(unknown), "Timed out or errored: possibly applied"),
			tile("Actors interrupted", formatCount(run.recoveries.length), `by ${run.faults.length} ${run.faults.length === 1 ? "fault" : "faults"}`),
			tile("Slowest recovery after a heal", formatSeconds(slowest)),
		),
		timeline({
			title: "Append latency",
			description: "Acknowledged appends, by the second they were invoked, on a log scale.",
			duration: run.duration_s,
			scale: "log",
			tickFormat: formatMsTick,
			faults: run.faults,
			series: [
				{ label: "p50", cls: "series-1", values: seconds.map((second) => second.p50_ms), format: formatMs },
				{ label: "p99", cls: "series-2", values: seconds.map((second) => second.p99_ms), format: formatMs },
			],
		}),
		timeline({
			title: "Appends per second",
			description: "By the second they were invoked. Unknown means the call timed out or errored and may have been applied.",
			duration: run.duration_s,
			scale: "linear",
			tickFormat: (value) => formatCount(value),
			faults: run.faults,
			series: [
				{ label: "Acknowledged", cls: "series-1", values: seconds.map((second) => second.acknowledged), format: formatCount },
				{ label: "Unknown or failed", cls: "series-2", values: seconds.map((second) => second.unknown + second.failed), format: formatCount },
			],
		}),
		run.recoveries.length > 0
			? outages(run.recoveries, run.faults, run.duration_s)
			: h("div", { class: "card empty" }, run.faults.length > 0 ? "No actor's calls failed during a fault." : "This run had no faults."),
		invariantsTable(run.invariants),
	);
}

async function loadRun(run: string): Promise<Run> {
	let pending = runs.get(run);
	if (!pending) {
		pending = fetch(`data/runs/${encodeURIComponent(run)}.json`).then((response) => {
			if (!response.ok) throw new Error(`data/runs/${run}.json: ${response.status}`);
			return response.json() as Promise<Run>;
		});
		runs.set(run, pending);
	}
	return pending;
}

async function render(index: IndexEntry[]): Promise<void> {
	if (!app) return;
	const state = readState(index);
	const groups = [...new Set(index.map((entry) => entry.group))].sort();
	const inGroup = index
		.filter((entry) => entry.group === state.group)
		.sort((a, b) => scenarioOrder(a.scenario) - scenarioOrder(b.scenario) || a.seed - b.seed);
	const run = state.run ?? inGroup[0]?.run ?? null;

	const filters = h(
		"div",
		{ class: "filters" },
		select("Runs", groups.map((group) => [group, groupName(group)]), state.group, (group) => writeState({ group, run: null })),
		select(
			"Run",
			inGroup.map((entry) => [entry.run, `${entry.scenario}, seed ${entry.seed}`]),
			run ?? "",
			(chosen) => writeState({ group: state.group, run: chosen }),
		),
	);
	slots.filters.replaceChildren(filters);
	slots.overview.replaceChildren(overview(index, state));
	if (!run) {
		slots.detail.replaceChildren();
		return;
	}
	const request = ++latest;
	slots.detail.classList.add("loading");
	let view: HTMLElement;
	try {
		view = runView(await loadRun(run));
	} catch (error) {
		view = h("div", { class: "card empty" }, `Couldn't load ${run}: ${String(error)}`);
	}
	// A slower load for an earlier choice mustn't replace the one now selected.
	if (request !== latest) return;
	slots.detail.replaceChildren(view);
	slots.detail.classList.remove("loading");
}

async function main(): Promise<void> {
	if (!app) return;
	let index: IndexEntry[];
	try {
		const response = await fetch("data/index.json");
		if (!response.ok) throw new Error(String(response.status));
		index = (await response.json()) as IndexEntry[];
	} catch {
		app.replaceChildren(
			h("div", { class: "card empty" }, "No run data yet. From the repository root, run ", h("code", {}, "make dashboard-data"), "."),
		);
		return;
	}
	app.replaceChildren(slots.filters, slots.overview, slots.detail);
	window.addEventListener("hashchange", () => void render(index));
	await render(index);
}

void main();
