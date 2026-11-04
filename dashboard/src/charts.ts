// The run charts, drawn as SVG: a timeline of per-second values with the fault
// windows shaded, and an outage timeline with one bar per interrupted actor.
// Each has a hover and keyboard readout, and a table view with the same values.

import { h, s } from "./dom";
import { formatSeconds } from "./format";
import type { Fault, Recovery } from "./types";

const MARGIN = { top: 22, right: 48, bottom: 30, left: 64 };
const PLOT_HEIGHT = 220;
/** Rough width of a character of 11px label text, to keep labels from overflowing. */
const CHAR_PX = 6.4;

interface Scale {
	(value: number): number;
	invert(px: number): number;
}

function linearScale(d0: number, d1: number, r0: number, r1: number): Scale {
	const k = d1 === d0 ? 0 : (r1 - r0) / (d1 - d0);
	const scale = (value: number) => r0 + (value - d0) * k;
	scale.invert = (px: number) => (k === 0 ? d0 : d0 + (px - r0) / k);
	return scale;
}

function logScale(d0: number, d1: number, r0: number, r1: number): Scale {
	const inner = linearScale(Math.log10(d0), Math.log10(d1), r0, r1);
	const scale = (value: number) => inner(Math.log10(Math.max(value, d0)));
	scale.invert = (px: number) => 10 ** inner.invert(px);
	return scale;
}

/** A round step, 1, 2 or 5 × a power of ten, for about `count` ticks. */
function tickStep(min: number, max: number, count: number): number {
	const raw = (max - min) / Math.max(count, 1);
	const power = 10 ** Math.floor(Math.log10(raw || 1));
	return [1, 2, 5, 10].map((m) => m * power).find((candidate) => candidate >= raw) ?? power * 10;
}

/** Round ticks inside [min, max]. */
function linearTicks(min: number, max: number, count: number): number[] {
	const step = tickStep(min, max, count);
	const ticks = [];
	for (let tick = Math.ceil(min / step) * step; tick <= max + step * 1e-9; tick += step) ticks.push(tick);
	return ticks;
}

/** Round ticks from zero up to the first tick at or above `max`, so the axis covers every value. */
function coveringTicks(max: number, count: number): number[] {
	const step = tickStep(0, max, count);
	const top = Math.max(Math.ceil(max / step), 1) * step;
	return linearTicks(0, top, Math.round(top / step));
}

export function faultLabel(fault: Fault): string {
	return fault.network ? `${fault.fault} ${fault.target} from ${fault.network}` : `${fault.fault} ${fault.target}`;
}

/** The faults in place at second `t`. */
function activeFaults(faults: Fault[], t: number, duration: number): Fault[] {
	return faults.filter((fault) => t >= fault.start_s && t <= (fault.heal_s ?? duration));
}

/** A figure with a title, a description, the chart, and a table-view toggle. */
function figure(
	title: string,
	description: string,
	plot: HTMLElement,
	table: () => HTMLTableElement,
	legend?: HTMLElement,
): HTMLElement {
	const tableView = h("div", { class: "table-view", hidden: true });
	const toggle = h("button", { type: "button", class: "ghost", "aria-expanded": "false" }, "Show table");
	toggle.addEventListener("click", () => {
		const open = tableView.hidden;
		if (open && !tableView.firstChild) tableView.append(table());
		tableView.hidden = !open;
		toggle.textContent = open ? "Hide table" : "Show table";
		toggle.setAttribute("aria-expanded", String(open));
	});
	return h(
		"figure",
		{ class: "card chart" },
		h("div", { class: "chart-head" }, h("div", {}, h("h3", {}, title), h("p", { class: "muted" }, description)), toggle),
		legend ?? null,
		plot,
		tableView,
	);
}

function tableOf(columns: string[], rows: (string | number)[][]): HTMLTableElement {
	return h(
		"table",
		{},
		h("thead", {}, h("tr", {}, ...columns.map((column) => h("th", { scope: "col" }, column)))),
		h("tbody", {}, ...rows.map((row) => h("tr", {}, ...row.map((cell) => h("td", {}, cell))))),
	);
}

/**
 * Calls `draw` with the plot's width now and whenever it changes. The plot gets
 * its final height first, so drawing never resizes what is being observed.
 */
function responsive(plot: HTMLElement, height: number, draw: (width: number) => void): void {
	plot.style.height = `${height}px`;
	let width = 0;
	new ResizeObserver(([entry]) => {
		const next = Math.floor(entry?.contentRect.width ?? 0);
		if (next > 0 && next !== width) {
			width = next;
			draw(width);
		}
	}).observe(plot);
}

function faultBands(group: SVGGElement, faults: Fault[], x: Scale, top: number, bottom: number, duration: number): void {
	for (const fault of faults) {
		const x0 = x(fault.start_s);
		const x1 = x(fault.heal_s ?? duration);
		group.append(s("rect", { class: "band", x: x0, y: top, width: Math.max(x1 - x0, 1), height: bottom - top }));
		const label = faultLabel(fault);
		// A label wider than its band is left to the tooltip rather than overflowing.
		if (label.length * CHAR_PX + 4 <= x1 - x0) {
			group.append(s("text", { class: "band-label", x: x0 + 4, y: top - 6 }, label));
		}
	}
}

function timeAxis(group: SVGGElement, x: Scale, duration: number, y: number): void {
	group.append(s("line", { class: "axis", x1: x(0), x2: x(duration), y1: y, y2: y }));
	for (const tick of linearTicks(0, duration, Math.max(2, Math.floor((x(duration) - x(0)) / 90)))) {
		group.append(s("text", { class: "tick", x: x(tick), y: y + 18, "text-anchor": "middle" }, `${tick} s`));
	}
}

function tooltipRow(value: string, label: string, keyClass?: string): HTMLElement {
	return h(
		"div",
		{ class: "tip-row" },
		keyClass ? h("span", { class: `key ${keyClass}` }) : null,
		h("strong", {}, value),
		h("span", { class: "muted" }, label),
	);
}

/** Puts the tooltip beside `px`, flipping to the left near the right edge. */
function placeTooltip(tip: HTMLElement, px: number, py: number, width: number): void {
	tip.hidden = false;
	const flip = px > width - 200;
	tip.style.left = flip ? "" : `${px + 12}px`;
	tip.style.right = flip ? `${width - px + 12}px` : "";
	tip.style.top = `${Math.max(py, 0)}px`;
}

export interface Series {
	label: string;
	/** The CSS class that colours the series: `series-1` or `series-2`. */
	cls: string;
	values: (number | null)[];
	format: (value: number) => string;
}

export interface TimelineOptions {
	title: string;
	description: string;
	duration: number;
	series: Series[];
	scale: "linear" | "log";
	tickFormat: (value: number) => string;
	faults: Fault[];
}

/** Per-second values, one line per series, against seconds since the run started. */
export function timeline(options: TimelineOptions): HTMLElement {
	const { series, faults, duration } = options;
	const seconds = series[0]?.values.length ?? 0;
	const plot = h("div", {
		class: "plot",
		tabindex: 0,
		role: "group",
		"aria-label": `${options.title}. Arrow keys move through the seconds; the table view has every value.`,
	});
	const tip = h("div", { class: "tooltip", role: "status", hidden: true });

	let cursor: number | null = null;
	let moveCursor: (index: number | null) => void = () => {};

	const height = MARGIN.top + PLOT_HEIGHT + MARGIN.bottom;
	responsive(plot, height, (width) => {
		// Room on the right for the longest direct label.
		const right = Math.max(MARGIN.right, Math.max(...series.map((one) => one.label.length)) * CHAR_PX + 12);
		const x = linearScale(0, Math.max(duration, 1), MARGIN.left, width - right);
		const values = series.flatMap((one) => one.values.filter((value): value is number => value !== null));
		const top = MARGIN.top;
		const bottom = MARGIN.top + PLOT_HEIGHT;
		let y: Scale;
		let ticks: number[];
		if (options.scale === "log") {
			const lo = 10 ** Math.floor(Math.log10(Math.max(Math.min(...values, 1), 0.1)));
			const hi = 10 ** Math.ceil(Math.log10(Math.max(...values, 10)));
			y = logScale(lo, hi, bottom, top);
			ticks = [];
			for (let tick = lo; tick <= hi; tick *= 10) ticks.push(tick);
		} else {
			const hi = Math.max(...values, 1);
			ticks = coveringTicks(hi, 4);
			y = linearScale(0, ticks[ticks.length - 1] ?? hi, bottom, top);
		}

		const svg = s("svg", { width, height, viewBox: `0 0 ${width} ${height}`, "aria-hidden": "true" });
		const chart = s("g");
		svg.append(chart);
		faultBands(chart, faults, x, top, bottom, duration);
		for (const tick of ticks) {
			chart.append(s("line", { class: "grid", x1: MARGIN.left, x2: width - right, y1: y(tick), y2: y(tick) }));
			chart.append(s("text", { class: "tick", x: MARGIN.left - 8, y: y(tick) + 4, "text-anchor": "end" }, options.tickFormat(tick)));
		}
		timeAxis(chart, x, duration, bottom);

		// Lines break where a second has no value.
		const ends: { y: number; label: string }[] = [];
		for (const one of series) {
			let path = "";
			let pen = false;
			let last: number | null = null;
			for (const [t, value] of one.values.entries()) {
				if (value === null) {
					pen = false;
					continue;
				}
				path += `${pen ? "L" : "M"}${x(t).toFixed(1)},${y(value).toFixed(1)}`;
				pen = true;
				last = value;
			}
			chart.append(s("path", { class: `line ${one.cls}`, d: path }));
			if (last !== null) ends.push({ y: y(last), label: one.label });
		}
		// Direct labels at the right edge, unless two would collide; the legend covers that.
		ends.sort((a, b) => a.y - b.y);
		ends.forEach((end, index) => {
			const previous = ends[index - 1];
			if (series.length > 1 && previous && end.y - previous.y < 12) return;
			chart.append(s("text", { class: "end-label", x: width - right + 6, y: end.y + 4 }, end.label));
		});

		const crosshair = s("line", { class: "crosshair", y1: top, y2: bottom, visibility: "hidden" });
		const dots = series.map((one) => s("circle", { class: `dot ${one.cls}`, r: 4, visibility: "hidden" }));
		chart.append(crosshair, ...dots);

		moveCursor = (index) => {
			cursor = index;
			if (index === null || seconds === 0) {
				crosshair.setAttribute("visibility", "hidden");
				dots.forEach((dot) => dot.setAttribute("visibility", "hidden"));
				tip.hidden = true;
				return;
			}
			const px = x(index);
			crosshair.setAttribute("x1", String(px));
			crosshair.setAttribute("x2", String(px));
			crosshair.setAttribute("visibility", "visible");
			let highest = bottom;
			series.forEach((one, i) => {
				const value = one.values[index];
				const dot = dots[i];
				if (!dot) return;
				if (value === null || value === undefined) {
					dot.setAttribute("visibility", "hidden");
					return;
				}
				dot.setAttribute("cx", String(px));
				dot.setAttribute("cy", String(y(value)));
				dot.setAttribute("visibility", "visible");
				highest = Math.min(highest, y(value));
			});
			tip.replaceChildren(
				h("div", { class: "tip-head" }, `${index} s`),
				...activeFaults(faults, index, duration).map((fault) => h("div", { class: "tip-fault" }, faultLabel(fault))),
				...series.map((one) => {
					const value = one.values[index];
					return tooltipRow(value === null || value === undefined ? "–" : one.format(value), one.label, one.cls);
				}),
			);
			placeTooltip(tip, px, highest - 10, width);
		};

		const overlay = s("rect", { class: "overlay", x: MARGIN.left, y: top, width: Math.max(width - MARGIN.left - right, 0), height: PLOT_HEIGHT });
		overlay.addEventListener("pointermove", (event) => {
			const bounds = svg.getBoundingClientRect();
			const t = Math.round(x.invert(event.clientX - bounds.left));
			moveCursor(Math.min(Math.max(t, 0), seconds - 1));
		});
		overlay.addEventListener("pointerleave", () => moveCursor(null));
		svg.append(overlay);
		plot.replaceChildren(svg, tip);
		if (cursor !== null) moveCursor(cursor);
	});

	plot.addEventListener("keydown", (event) => {
		const step = event.shiftKey ? 10 : 1;
		const start = cursor ?? 0;
		if (event.key === "ArrowRight") moveCursor(Math.min(start + step, seconds - 1));
		else if (event.key === "ArrowLeft") moveCursor(Math.max(start - step, 0));
		else if (event.key === "Escape") moveCursor(null);
		else return;
		event.preventDefault();
	});
	plot.addEventListener("focus", () => moveCursor(cursor ?? 0));
	plot.addEventListener("blur", () => moveCursor(null));

	const legend =
		series.length > 1
			? h(
					"div",
					{ class: "legend" },
					...series.map((one) => h("span", { class: "legend-item" }, h("span", { class: `key ${one.cls}` }), one.label)),
				)
			: undefined;
	const table = () =>
		tableOf(
			["Second", ...series.map((one) => one.label)],
			Array.from({ length: seconds }, (_, t) => [
				t,
				...series.map((one) => {
					const value = one.values[t];
					return value === null || value === undefined ? "–" : one.format(value);
				}),
			]),
		);
	return figure(options.title, options.description, plot, table, legend);
}

const ROW = 14;
const BAR = 8;
const GROUP_GAP = 12;

/**
 * One bar per actor a fault interrupted, from its first failed call to the end of
 * the call that answered again, grouped by fault, against seconds since the run
 * started. The fault windows are shaded, so a bar running past its band waited
 * for the heal.
 */
export function outages(recoveries: Recovery[], faults: Fault[], duration: number): HTMLElement {
	const rows = [...recoveries].sort((a, b) => a.fault - b.fault || a.failed_s - b.failed_s);
	const groups = new Set(rows.map((row) => row.fault)).size;
	const plotHeight = rows.length * ROW + Math.max(groups - 1, 0) * GROUP_GAP;
	const plot = h("div", {
		class: "plot",
		role: "group",
		"aria-label": `Outages: ${rows.length} interrupted actors. Tab moves through the bars; the table view has every value.`,
	});
	const tip = h("div", { class: "tooltip", role: "status", hidden: true });

	const height = MARGIN.top + plotHeight + MARGIN.bottom;
	responsive(plot, height, (width) => {
		const left = 92;
		const x = linearScale(0, Math.max(duration, 1), left, width - MARGIN.right);
		const top = MARGIN.top;
		const bottom = MARGIN.top + plotHeight;
		const svg = s("svg", { width, height, viewBox: `0 0 ${width} ${height}` });
		const chart = s("g");
		svg.append(chart);
		faultBands(chart, faults, x, top, bottom, duration);
		for (const tick of linearTicks(0, duration, Math.max(2, Math.floor((width - left - MARGIN.right) / 90)))) {
			chart.append(s("line", { class: "grid", x1: x(tick), x2: x(tick), y1: top, y2: bottom }));
		}
		timeAxis(chart, x, duration, bottom);

		let offset = top;
		let previousFault: number | null = null;
		for (const row of rows) {
			if (previousFault !== null && row.fault !== previousFault) offset += GROUP_GAP;
			previousFault = row.fault;
			const fault = faults[row.fault];
			const y = offset + (ROW - BAR) / 2;
			const x0 = x(row.failed_s);
			const x1 = Math.max(x(row.answered_s), x0 + 2);
			chart.append(s("text", { class: "row-label", x: left - 8, y: y + BAR - 1, "text-anchor": "end" }, row.actor));
			const bar = s("rect", { class: "bar series-1", x: x0, y, width: x1 - x0, height: BAR, rx: 2 });
			// The hit target covers the whole row band and at least 24px across.
			const hitWidth = Math.max(x1 - x0, 24);
			const hit = s("rect", {
				class: "hit",
				x: (x0 + x1) / 2 - hitWidth / 2,
				y: offset,
				width: hitWidth,
				height: ROW,
				tabindex: 0,
				role: "img",
				"aria-label": `${row.actor}: out for ${formatSeconds(row.outage_s)}, answered ${formatSeconds(row.after_heal_s, true)} relative to the heal`,
			});
			const show = () => {
				bar.classList.add("active");
				const relative =
					row.after_heal_s < 0
						? `answered ${formatSeconds(-row.after_heal_s)} before the heal`
						: `answered ${formatSeconds(row.after_heal_s)} after the heal`;
				tip.replaceChildren(
					h("div", { class: "tip-head" }, row.actor),
					h("div", { class: "tip-fault" }, fault ? faultLabel(fault) : `fault ${row.fault + 1}`),
					tooltipRow(formatSeconds(row.outage_s), "without an answer"),
					tooltipRow(formatSeconds(row.failed_s), "first failed call"),
					h("div", { class: "muted tip-note" }, relative),
				);
				placeTooltip(tip, x1, y - 4, width);
			};
			const hide = () => {
				bar.classList.remove("active");
				tip.hidden = true;
			};
			hit.addEventListener("pointerenter", show);
			hit.addEventListener("pointerleave", hide);
			hit.addEventListener("focus", show);
			hit.addEventListener("blur", hide);
			chart.append(bar, hit);
			offset += ROW;
		}
		plot.replaceChildren(svg, tip);
	});

	const table = () =>
		tableOf(
			["Fault", "Actor", "First failed call", "Answered again", "Without an answer", "Relative to the heal"],
			rows.map((row) => {
				const fault = faults[row.fault];
				return [
					fault ? `${row.fault + 1}: ${faultLabel(fault)}` : String(row.fault + 1),
					row.actor,
					formatSeconds(row.failed_s),
					formatSeconds(row.answered_s),
					formatSeconds(row.outage_s),
					formatSeconds(row.after_heal_s, true),
				];
			}),
		);
	return figure(
		"Outages",
		"One bar per actor a fault interrupted, from its first failed call until a call answered again. Shading marks each fault; a bar past its shading waited for the heal.",
		plot,
		table,
	);
}
