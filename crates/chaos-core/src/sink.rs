//! What the effects sink recorded during a run, archived next to the history.
//!
//! The sink and the nodes stamp rows with `CLOCK_MONOTONIC`, which every container
//! on one host shares and which never steps, unlike the wall clock. `offset_ns`,
//! measured when the run starts, maps those readings onto the history's `mono_ns`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SinkRecords {
    /// Added to a sink or node reading, gives the history's `mono_ns`.
    pub offset_ns: i64,
    pub effects: Vec<EffectRow>,
    pub alarms: Vec<AlarmRow>,
    pub heartbeats: Vec<HeartbeatRow>,
}

/// A tool call, keyed `session/turn/tool`, stamped when the sink received it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectRow {
    pub key: String,
    pub node: String,
    pub mono_ns: u64,
}

/// A follow-up alarm, keyed `session/turn`. `due_mono_ns` is when the workflow
/// asked for it to fire; `mono_ns` is when the sink heard that it did.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlarmRow {
    pub key: String,
    pub node: String,
    pub due_mono_ns: u64,
    pub mono_ns: u64,
}

/// A session's heartbeat, stamped by the node when it was taken, so a backlog a
/// cut-off node delivers later still lands at the right time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeartbeatRow {
    pub key: String,
    pub node: String,
    pub sent_mono_ns: u64,
}

impl SinkRecords {
    /// A sink or node reading on the history's clock.
    pub fn history_ns(&self, mono_ns: u64) -> i64 {
        mono_ns as i64 + self.offset_ns
    }
}
