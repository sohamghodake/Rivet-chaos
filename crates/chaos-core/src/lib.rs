//! Shared types: the history a run records, and the scenario that drives it.

pub mod event;
pub mod scenario;

pub use event::{Body, Event, Fault, Header, Op, Proc, SCHEMA_VERSION};
pub use scenario::{Checks, FaultSpec, Scenario, ScenarioError, Targets, Workload};
