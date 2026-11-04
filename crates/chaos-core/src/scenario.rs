//! A scenario file sets the workload, the fault schedule and the check deadlines.
//! Unknown keys are an error, so a typo can't silently change a run.

use std::fmt;

use serde::Deserialize;

use crate::{Fault, Network};

/// Why a scenario file was rejected.
#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioError(pub String);

impl fmt::Display for ScenarioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ScenarioError {}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub name: String,
    pub seed: u64,
    pub duration_s: u64,
    pub workload: Workload,
    #[serde(default)]
    pub nemesis: Vec<FaultSpec>,
    pub checks: Checks,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Workload {
    pub ledgers: u32,
    pub processes: u32,
    pub pacing_ms: [u64; 2],
    pub call_timeout_ms: u64,
    /// Share of ledger calls that are reads, which I4 checks.
    #[serde(default = "default_read_percent")]
    pub read_percent: u32,
    /// Agent sessions, each sent a new turn every `turn_pacing_ms`.
    #[serde(default)]
    pub agent_sessions: u32,
    #[serde(default = "default_turn_pacing_ms")]
    pub turn_pacing_ms: [u64; 2],
}

fn default_read_percent() -> u32 {
    10
}

fn default_turn_pacing_ms() -> [u64; 2] {
    [2_000, 4_000]
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FaultSpec {
    pub fault: Fault,
    /// One service, or a list to pick from with the run's seed each time. For
    /// `kill-in-step`, the nodes it may kill.
    pub target: Targets,
    /// The network a `disconnect` cuts; required for it and rejected otherwise.
    #[serde(default)]
    pub network: Option<Network>,
    pub at_s: u64,
    pub heal_after_s: u64,
    pub repeat_every_s: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Targets {
    One(String),
    Any(Vec<String>),
}

impl Targets {
    pub fn choices(&self) -> &[String] {
        match self {
            Targets::One(target) => std::slice::from_ref(target),
            Targets::Any(targets) => targets,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Checks {
    pub quiesce_s: u64,
    pub recovery_deadline_s: u64,
}

impl FaultSpec {
    /// Start times in seconds from the run's start, all before `duration_s`.
    pub fn start_times(&self, duration_s: u64) -> Vec<u64> {
        match self.repeat_every_s {
            None if self.at_s < duration_s => vec![self.at_s],
            None => vec![],
            Some(every) => (self.at_s..duration_s).step_by(every as usize).collect(),
        }
    }
}

impl Scenario {
    pub fn parse(text: &str) -> Result<Self, ScenarioError> {
        let scenario: Scenario = toml::from_str(text).map_err(|error| ScenarioError(error.to_string()))?;
        scenario.validate()?;
        Ok(scenario)
    }

    fn validate(&self) -> Result<(), ScenarioError> {
        let fail = |message: String| Err(ScenarioError(message));
        let workload = &self.workload;
        if workload.ledgers == 0 || workload.processes == 0 {
            return fail("workload needs at least one ledger and one process".into());
        }
        if workload.pacing_ms[0] > workload.pacing_ms[1] || workload.turn_pacing_ms[0] > workload.turn_pacing_ms[1] {
            return fail("workload.pacing_ms and workload.turn_pacing_ms must be [min, max]".into());
        }
        if workload.read_percent > 100 {
            return fail("workload.read_percent can't exceed 100".into());
        }
        for (index, spec) in self.nemesis.iter().enumerate() {
            if spec.target.choices().is_empty() {
                return fail(format!("nemesis {index} has no target"));
            }
            match (spec.fault, spec.network) {
                (Fault::Disconnect, None) => return fail(format!("nemesis {index} disconnects but names no network")),
                (Fault::Disconnect, Some(_)) | (_, None) => {}
                (fault, Some(_)) => return fail(format!("nemesis {index} is a {fault}, which takes no network")),
            }
            if spec.fault == Fault::KillInStep && workload.agent_sessions == 0 {
                return fail(format!("nemesis {index} kills inside workflow steps, so it needs agent_sessions"));
            }
            if let Some(every) = spec.repeat_every_s
                && every <= spec.heal_after_s
            {
                return fail(format!(
                    "nemesis {index} repeats every {every} s but heals after {} s, so its faults would overlap",
                    spec.heal_after_s
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
        name = "instance-crash"
        seed = 42
        duration_s = 600

        [workload]
        ledgers = 50
        processes = 16
        pacing_ms = [50, 200]
        call_timeout_ms = 5000

        [[nemesis]]
        fault = "kill"
        target = ["engine-1", "engine-2"]
        at_s = 120
        heal_after_s = 30
        repeat_every_s = 120

        [checks]
        quiesce_s = 30
        recovery_deadline_s = 300
    "#;

    #[test]
    fn parses_the_example() {
        let scenario = Scenario::parse(EXAMPLE).unwrap();
        assert_eq!(scenario.workload.pacing_ms, [50, 200]);
        assert_eq!(scenario.nemesis[0].fault, Fault::Kill);
        assert_eq!(scenario.nemesis[0].target.choices().len(), 2);
    }

    #[test]
    fn single_target_is_accepted() {
        let text = EXAMPLE.replace(r#"["engine-1", "engine-2"]"#, r#""node-a""#);
        let scenario = Scenario::parse(&text).unwrap();
        assert_eq!(scenario.nemesis[0].target.choices(), ["node-a"]);
    }

    #[test]
    fn typos_are_rejected() {
        let text = EXAMPLE.replace("quiesce_s", "quiesce_seconds");
        assert!(Scenario::parse(&text).is_err());
    }

    #[test]
    fn overlapping_repeats_are_rejected() {
        let text = EXAMPLE.replace("repeat_every_s = 120", "repeat_every_s = 30");
        assert!(Scenario::parse(&text).unwrap_err().0.contains("overlap"));
    }

    #[test]
    fn workload_defaults_fill_in() {
        let workload = Scenario::parse(EXAMPLE).unwrap().workload;
        assert_eq!((workload.read_percent, workload.agent_sessions), (10, 0));
        assert_eq!(workload.turn_pacing_ms, [2_000, 4_000]);
    }

    #[test]
    fn disconnect_needs_exactly_its_network() {
        let disconnect = EXAMPLE.replace(r#"fault = "kill""#, r#"fault = "disconnect""#);
        assert!(Scenario::parse(&disconnect).unwrap_err().0.contains("names no network"));
        let with_network = disconnect.replace("at_s = 120", "network = \"peer\"\nat_s = 120");
        assert_eq!(Scenario::parse(&with_network).unwrap().nemesis[0].network, Some(Network::Peer));
        let kill_with_network = EXAMPLE.replace("at_s = 120", "network = \"peer\"\nat_s = 120");
        assert!(Scenario::parse(&kill_with_network).unwrap_err().0.contains("takes no network"));
    }

    #[test]
    fn kill_in_step_needs_sessions() {
        let text = EXAMPLE.replace(r#"fault = "kill""#, r#"fault = "kill-in-step""#);
        assert!(Scenario::parse(&text).unwrap_err().0.contains("agent_sessions"));
        let text = text.replace("call_timeout_ms = 5000", "call_timeout_ms = 5000\nagent_sessions = 4");
        assert_eq!(Scenario::parse(&text).unwrap().nemesis[0].fault, Fault::KillInStep);
    }

    #[test]
    fn start_times_stop_before_the_end() {
        let spec = &Scenario::parse(EXAMPLE).unwrap().nemesis[0];
        assert_eq!(spec.start_times(600), [120, 240, 360, 480]);
        assert_eq!(spec.start_times(120), Vec::<u64>::new());
    }
}
