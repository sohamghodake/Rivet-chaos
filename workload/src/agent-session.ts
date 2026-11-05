import { hostname } from "node:os";
import { actor, event, queue } from "rivetkit";
import { db } from "rivetkit/db";
import { workflow } from "rivetkit/workflow";

const NODE_ID = process.env.NODE_ID ?? hostname();
const SINK_URL = process.env.SINK_URL ?? "http://localhost:6510";
const LLM_URL = process.env.LLM_URL ?? "http://localhost:6511";
const FOLLOW_UP_MS = Number(process.env.FOLLOW_UP_MS ?? 1_000);
const HEARTBEAT_MS = 250;
// Twenty minutes of heartbeats, held while the sink can't be reached.
const HEARTBEAT_BACKLOG = 4_800;
const HTTP_TIMEOUT_MS = 5_000;

type UserMessage = { turn: number; text: string };
type Completion = { reply: string; tool_calls: { name: string }[] };
type Heartbeat = { key: string; node: string; sent_mono_ns: number };

// CLOCK_MONOTONIC, which every container on the host shares and which never
// steps, unlike Date.now(). The sink stamps its rows with the same clock.
function monoNs(): number {
	return Number(process.hrtime.bigint());
}

async function post<T>(url: string, body: unknown): Promise<T> {
	const res = await fetch(url, {
		method: "POST",
		headers: { "content-type": "application/json" },
		body: JSON.stringify(body),
		signal: AbortSignal.timeout(HTTP_TIMEOUT_MS),
	});
	if (!res.ok) throw new Error(`POST ${url}: ${res.status}`);
	return (await res.json()) as T;
}

export const agentSession = actor({
	db: db({
		onMigrate: async (db) => {
			await db.execute(`
				CREATE TABLE IF NOT EXISTS turns (
					turn INTEGER PRIMARY KEY,
					reply TEXT NOT NULL,
					tool_calls INTEGER NOT NULL
				)
			`);
		},
	}),
	queues: {
		message: queue<UserMessage>(),
	},
	events: {
		token: event<{ turn: number; index: number; token: string }>(),
	},
	// Workflow steps have no `schedule`, so it is captured here for them.
	createVars: (c) => ({
		heartbeat: undefined as NodeJS.Timeout | undefined,
		// Heartbeats not yet delivered, so a node cut off from the sink still
		// reports when it was running once it can reach the sink again.
		backlog: [] as Heartbeat[],
		flushing: false,
		schedule: c.schedule,
	}),
	onWake: (c) => {
		const key = c.key[0];
		const vars = c.vars;
		// Two nodes heartbeating for one key at once is the split-brain signal.
		vars.heartbeat = setInterval(() => {
			vars.backlog.push({ key, node: NODE_ID, sent_mono_ns: monoNs() });
			vars.backlog.splice(0, vars.backlog.length - HEARTBEAT_BACKLOG);
			if (vars.flushing) return;
			vars.flushing = true;
			const batch = vars.backlog.splice(0);
			post(`${SINK_URL}/heartbeats`, batch)
				.catch(() => vars.backlog.unshift(...batch))
				.finally(() => {
					vars.flushing = false;
				});
		}, HEARTBEAT_MS);
		// A node that loses the control plane stops its actors without calling
		// onSleep; the abort signal fires on every way an actor stops.
		c.abortSignal.addEventListener("abort", () => clearInterval(vars.heartbeat), { once: true });
	},
	onSleep: (c) => {
		clearInterval(c.vars.heartbeat);
	},
	onDestroy: (c) => {
		clearInterval(c.vars.heartbeat);
	},
	run: workflow(async (ctx) => {
		await ctx.loop("turns", async (loopCtx) => {
			const message = await loopCtx.queue.next("wait-message", {
				names: ["message"],
			});
			const { turn, text } = message.body as UserMessage;
			const session = loopCtx.key[0];

			const completion = await loopCtx.step("llm", () =>
				post<Completion>(`${LLM_URL}/v1/complete`, { session, turn, text }),
			);

			for (const [index, call] of completion.tool_calls.entries()) {
				await loopCtx.step(`tool-${index}`, async () => {
					await post(`${SINK_URL}/effects`, {
						key: `${session}/${turn}/${call.name}`,
						node: NODE_ID,
					});
				});
			}

			// The step can rerun after a crash, so the insert has to tolerate that.
			await loopCtx.step("save-reply", async (step) => {
				await step.db.execute(
					"INSERT INTO turns (turn, reply, tool_calls) VALUES (?, ?, ?) ON CONFLICT (turn) DO NOTHING",
					turn,
					completion.reply,
					completion.tool_calls.length,
				);
				for (const [index, token] of completion.reply.split(" ").entries()) {
					step.broadcast("token", { turn, index, token });
				}
			});

			await loopCtx.step("schedule-follow-up", async (step) => {
				await step.vars.schedule.after(
					FOLLOW_UP_MS,
					"followUp",
					turn,
					Date.now() + FOLLOW_UP_MS,
					monoNs() + FOLLOW_UP_MS * 1_000_000,
				);
			});
		});
	}),
	actions: {
		turns: async (c) =>
			(await c.db.execute(
				"SELECT turn, reply, tool_calls FROM turns ORDER BY turn",
			)) as { turn: number; reply: string; tool_calls: number }[],
		followUp: async (c, turn: number, dueMs: number, dueMonoNs: number) => {
			await post(`${SINK_URL}/alarms`, {
				key: `${c.key[0]}/${turn}`,
				node: NODE_ID,
				due_ms: dueMs,
				due_mono_ns: dueMonoNs,
			});
		},
	},
});
