//! One line of history per event. Every client call is an `invoke` followed by
//! exactly one result: `ok`, `fail` (definitely not applied) or `info` (outcome
//! unknown, so possibly applied). The nemesis writes its fault windows into the
//! same log.

use std::{collections::BTreeMap, fmt};

use serde::{Deserialize, Serialize};

/// Bumped whenever a reader of an older history would misread a newer one.
pub const SCHEMA_VERSION: u32 = 1;

/// The first line of every history: what ran, with what, on what.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Header {
    pub schema: u32,
    pub run: String,
    pub scenario: String,
    pub seed: u64,
    pub started_wall_ms: u64,
    /// The scenario file as it was run, so the run can be repeated exactly.
    pub scenario_toml: String,
    pub versions: BTreeMap<String, String>,
    pub host: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub run: String,
    pub seq: u64,
    pub proc: Proc,
    #[serde(flatten)]
    pub body: Body,
    pub mono_ns: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wall_ms: Option<u64>,
}

/// A logical client process, which makes one call at a time, or the nemesis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "ProcRepr", into = "ProcRepr")]
pub enum Proc {
    Client(u32),
    Nemesis,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ProcRepr {
    Client(u32),
    Named(String),
}

impl TryFrom<ProcRepr> for Proc {
    type Error = String;

    fn try_from(repr: ProcRepr) -> Result<Self, Self::Error> {
        match repr {
            ProcRepr::Client(id) => Ok(Proc::Client(id)),
            ProcRepr::Named(name) if name == "nemesis" => Ok(Proc::Nemesis),
            ProcRepr::Named(name) => Err(format!("unknown process {name:?}")),
        }
    }
}

impl From<Proc> for ProcRepr {
    fn from(proc: Proc) -> Self {
        match proc {
            Proc::Client(id) => ProcRepr::Client(id),
            Proc::Nemesis => ProcRepr::Named("nemesis".into()),
        }
    }
}

impl fmt::Display for Proc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Proc::Client(id) => write!(f, "{id}"),
            Proc::Nemesis => f.write_str("nemesis"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Body {
    Invoke {
        #[serde(flatten)]
        op: Op,
    },
    Ok {
        #[serde(flatten)]
        op: Op,
        /// What a read returned.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        op_ids: Option<Vec<String>>,
        /// An append that found its op_id already written: applied, by an
        /// earlier attempt of the same call.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        duplicate: bool,
    },
    Fail {
        #[serde(flatten)]
        op: Op,
        error: String,
    },
    Info {
        #[serde(flatten)]
        op: Op,
        error: String,
    },
    FaultStart {
        fault: Fault,
        target: String,
    },
    FaultHeal {
        fault: Fault,
        target: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "f", rename_all = "kebab-case")]
pub enum Op {
    Append {
        actor: String,
        op_id: String,
        value: i64,
    },
    Read {
        actor: String,
    },
}

impl Op {
    pub fn actor(&self) -> &str {
        match self {
            Op::Append { actor, .. } | Op::Read { actor } => actor,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Fault {
    /// F1: SIGKILL a container, then start it again.
    Kill,
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fault::Kill => f.write_str("kill"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The three example lines from the plan's correctness model.
    const INVOKE: &str = r#"{"run":"node-kill-s42","seq":1042,"proc":7,"type":"invoke","f":"append","actor":"ledger/17","op_id":"p7-000213","value":213,"mono_ns":81234567890,"wall_ms":1794043331000}"#;
    const FAULT: &str = r#"{"run":"node-kill-s42","seq":1044,"proc":"nemesis","type":"fault-start","fault":"kill","target":"node-a","mono_ns":81240000000}"#;

    #[test]
    fn plan_examples_round_trip_unchanged() {
        for line in [INVOKE, FAULT] {
            let event: Event = serde_json::from_str(line).unwrap();
            assert_eq!(serde_json::to_string(&event).unwrap(), line);
        }
    }

    #[test]
    fn invoke_decodes_into_typed_fields() {
        let event: Event = serde_json::from_str(INVOKE).unwrap();
        assert_eq!(event.proc, Proc::Client(7));
        assert_eq!(
            event.body,
            Body::Invoke {
                op: Op::Append {
                    actor: "ledger/17".into(),
                    op_id: "p7-000213".into(),
                    value: 213,
                },
            }
        );
    }

    #[test]
    fn read_results_and_duplicates_round_trip() {
        let event = Event {
            run: "r".into(),
            seq: 3,
            proc: Proc::Client(1),
            body: Body::Ok {
                op: Op::Read { actor: "ledger/1".into() },
                op_ids: Some(vec!["p0-000001".into()]),
                duplicate: true,
            },
            mono_ns: 5,
            wall_ms: None,
        };
        let line = serde_json::to_string(&event).unwrap();
        assert_eq!(serde_json::from_str::<Event>(&line).unwrap(), event);
    }

    #[test]
    fn unknown_named_process_is_rejected() {
        let line = FAULT.replace("\"nemesis\"", "\"gremlin\"");
        assert!(serde_json::from_str::<Event>(&line).is_err());
    }
}
