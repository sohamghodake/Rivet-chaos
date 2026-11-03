import { setup } from "rivetkit";
import { agentSession } from "./agent-session.ts";
import { ledger } from "./ledger.ts";

export const registry = setup({
	use: { ledger, "agent-session": agentSession },
});

registry.start();
