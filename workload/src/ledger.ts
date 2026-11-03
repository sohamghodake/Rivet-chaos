import { actor } from "rivetkit";
import { db } from "rivetkit/db";

// Unique op IDs make lost and duplicated writes visible to the checker.
export const ledger = actor({
	db: db({
		onMigrate: async (db) => {
			await db.execute(`
				CREATE TABLE IF NOT EXISTS entries (
					op_id TEXT PRIMARY KEY,
					value INTEGER,
					written_at INTEGER
				)
			`);
		},
	}),
	actions: {
		// A single autocommit INSERT: once this returns, the write is expected
		// to be on a majority of disks.
		append: async (c, opId: string, value: number) => {
			const inserted = await c.db.execute(
				"INSERT INTO entries (op_id, value, written_at) VALUES (?, ?, ?) ON CONFLICT (op_id) DO NOTHING RETURNING op_id",
				opId,
				value,
				Date.now(),
			);
			return inserted.length > 0 ? "inserted" : "duplicate";
		},
		read: async (c) => {
			const rows = (await c.db.execute("SELECT op_id FROM entries")) as {
				op_id: string;
			}[];
			return rows.map((row) => row.op_id);
		},
	},
});
