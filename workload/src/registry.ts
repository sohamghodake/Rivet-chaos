import { setup } from "rivetkit";
import { agentSession } from "./agent-session.ts";
import { alarmProbe } from "./alarm-probe.ts";
import { ledger } from "./ledger.ts";

export const registry = setup({
	use: { ledger, "agent-session": agentSession, "alarm-probe": alarmProbe },
});

registry.start();
