//! Shared types: the history a run records, the scenario that drives it, and the
//! sink records it is judged against.

pub mod event;
pub mod scenario;
pub mod sink;

pub use event::{Body, Event, Fault, Header, Network, Op, Proc, SCHEMA_VERSION, Turn};
pub use scenario::{Checks, FaultSpec, Scenario, ScenarioError, Targets, Workload};
pub use sink::{AlarmRow, EffectRow, HeartbeatRow, SinkRecords};
