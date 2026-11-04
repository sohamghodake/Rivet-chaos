const counts = new Intl.NumberFormat("en-US");

export function formatCount(value: number): string {
	return counts.format(value);
}

/** 61.0 s; a sign only when asked for, for times relative to a heal. */
export function formatSeconds(value: number, signed = false): string {
	const text = `${Math.abs(value).toFixed(1)} s`;
	if (!signed) return value < 0 ? `−${text}` : text;
	return value < 0 ? `−${text}` : `+${text}`;
}

/** 7.2 ms under a second, 1.24 s above. */
export function formatMs(value: number): string {
	if (value >= 1000) return `${(value / 1000).toFixed(2)} s`;
	if (value >= 100) return `${Math.round(value)} ms`;
	return `${value.toFixed(1)} ms`;
}

/** Axis ticks: 1 ms, 10 ms, 100 ms, 1 s. */
export function formatMsTick(value: number): string {
	return value >= 1000 ? `${value / 1000} s` : `${value} ms`;
}

export function formatDate(wallMs: number): string {
	return new Date(wallMs).toISOString().replace("T", " ").slice(0, 16) + " UTC";
}
