import { hostname } from "node:os";
import { actor } from "rivetkit";

const NODE_ID = process.env.NODE_ID ?? hostname();
const SINK_URL = process.env.SINK_URL ?? "http://localhost:6510";

function monoNs(): number {
	return Number(process.hrtime.bigint());
}

// Books alarms from a plain action, without a workflow, so a booking that was
// acknowledged and never fires is the scheduler's doing alone. Used by
// repro/lost-alarms; the harness doesn't drive it.
export const alarmProbe = actor({
	actions: {
		book: async (c, id: string, delayMs: number) => {
			await c.schedule.after(delayMs, "fire", id, Date.now() + delayMs, monoNs() + delayMs * 1_000_000);
			return "booked";
		},
		fire: async (c, id: string, dueMs: number, dueMonoNs: number) => {
			const response = await fetch(`${SINK_URL}/alarms`, {
				method: "POST",
				headers: { "content-type": "application/json" },
				body: JSON.stringify({ key: `probe/${c.key[0]}/${id}`, node: NODE_ID, due_ms: dueMs, due_mono_ns: dueMonoNs }),
				signal: AbortSignal.timeout(5_000),
			});
			if (!response.ok) throw new Error(`POST ${SINK_URL}/alarms: ${response.status}`);
		},
	},
});
