//! Logical client processes that call actor actions with pacing and timeouts.
//!
//! Each process makes one call at a time and records it as an `invoke` followed
//! by exactly one result. A process whose call ends in `info` retires, and its
//! slot carries on under a new process ID, so the order of calls within any one
//! process stays meaningful.

use std::{
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use chaos_core::{Body, Op, Proc};
use chaos_history::Recorder;
use rand::{Rng, SeedableRng, rngs::StdRng};
use rivetkit_client::{Client, GetOrCreateOptions, handle::ActorHandle};
use serde_json::{Value, json};
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

pub fn ledger_actor(index: usize) -> String {
    format!("ledger/{index}")
}

/// Hands out process IDs. Shared by the load and the final reads, so no two
/// processes in a run ever share one.
#[derive(Clone, Default)]
pub struct Procs(Arc<AtomicU32>);

impl Procs {
    pub fn next(&self) -> Proc {
        Proc::Client(self.0.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Clone)]
pub struct Ledgers {
    handles: Arc<Vec<ActorHandle>>,
}

impl Ledgers {
    pub fn new(client: &Client, count: u32) -> Result<Self> {
        let handles = (0..count)
            .map(|index| client.get_or_create("ledger", vec![index.to_string()], GetOrCreateOptions::default()))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            handles: Arc::new(handles),
        })
    }

    pub fn len(&self) -> usize {
        self.handles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.handles.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    Ok(Value),
    Fail,
    Info,
}

/// Only a refused connection proves a request never left, so only that is a
/// `fail`. Anything else might have been applied, so it's `info`.
fn definitely_not_sent(error: &anyhow::Error) -> bool {
    error
        .chain()
        .any(|cause| cause.downcast_ref::<reqwest::Error>().is_some_and(reqwest::Error::is_connect))
}

async fn call(
    handle: &ActorHandle,
    recorder: &Recorder,
    proc: Proc,
    op: Op,
    action: &str,
    args: Vec<Value>,
    timeout: Duration,
) -> Result<Outcome> {
    recorder.record(proc, Body::Invoke { op: op.clone() }, true)?;
    let (body, outcome) = match tokio::time::timeout(timeout, handle.action(action, args)).await {
        Err(_) => (
            Body::Info {
                op,
                error: format!("timed out after {} ms", timeout.as_millis()),
            },
            Outcome::Info,
        ),
        Ok(Err(error)) if definitely_not_sent(&error) => (
            Body::Fail {
                op,
                error: format!("{error:#}"),
            },
            Outcome::Fail,
        ),
        Ok(Err(error)) => (
            Body::Info {
                op,
                error: format!("{error:#}"),
            },
            Outcome::Info,
        ),
        Ok(Ok(value)) => {
            let body = match &op {
                Op::Append { .. } => Body::Ok {
                    duplicate: value == "duplicate",
                    op,
                    op_ids: None,
                },
                Op::Read { .. } => Body::Ok {
                    op_ids: Some(serde_json::from_value(value.clone()).context("read result")?),
                    op,
                    duplicate: false,
                },
            };
            (body, Outcome::Ok(value))
        }
    };
    recorder.record(proc, body, false)?;
    Ok(outcome)
}

#[derive(Debug, Clone)]
pub struct LoadConfig {
    pub processes: u32,
    pub pacing_ms: [u64; 2],
    pub call_timeout: Duration,
    pub seed: u64,
}

/// Appends to random ledgers from `processes` slots until `stop` is cancelled,
/// then waits for the calls in flight to finish.
pub async fn run_load(
    ledgers: Ledgers,
    recorder: Recorder,
    procs: Procs,
    config: LoadConfig,
    stop: CancellationToken,
) -> Result<()> {
    let mut slots = JoinSet::new();
    for slot in 0..config.processes {
        let (ledgers, recorder, procs, config, stop) =
            (ledgers.clone(), recorder.clone(), procs.clone(), config.clone(), stop.clone());
        slots.spawn(async move {
            // Each slot gets its own stream of choices from the run's seed.
            let slot_seed = config.seed ^ (u64::from(slot) + 1).wrapping_mul(0x9e37_79b9_7f4a_7c15);
            let mut rng = StdRng::seed_from_u64(slot_seed);
            let mut proc = procs.next();
            let mut count = 0u32;
            while !stop.is_cancelled() {
                let index = rng.random_range(0..ledgers.len());
                let Proc::Client(id) = proc else { unreachable!() };
                let op_id = format!("p{id}-{count:06}");
                let op = Op::Append {
                    actor: ledger_actor(index),
                    op_id: op_id.clone(),
                    value: i64::from(count),
                };
                let args = vec![json!(op_id), json!(count)];
                let outcome =
                    call(&ledgers.handles[index], &recorder, proc, op, "append", args, config.call_timeout).await?;
                count += 1;
                if outcome == Outcome::Info {
                    proc = procs.next();
                    count = 0;
                }
                let pause = Duration::from_millis(rng.random_range(config.pacing_ms[0]..=config.pacing_ms[1]));
                tokio::select! {
                    _ = stop.cancelled() => break,
                    _ = tokio::time::sleep(pause) => {}
                }
            }
            anyhow::Ok(())
        });
    }
    while let Some(result) = slots.join_next().await {
        result??;
    }
    Ok(())
}

/// Reads every ledger, retrying each until it succeeds or `deadline` passes.
/// Returns the ledgers that never answered.
pub async fn final_reads(
    ledgers: Ledgers,
    recorder: Recorder,
    procs: Procs,
    call_timeout: Duration,
    deadline: Duration,
) -> Result<Vec<String>> {
    let started = Instant::now();
    let mut reads = JoinSet::new();
    for index in 0..ledgers.len() {
        let (ledgers, recorder, procs) = (ledgers.clone(), recorder.clone(), procs.clone());
        reads.spawn(async move {
            let mut proc = procs.next();
            loop {
                let op = Op::Read {
                    actor: ledger_actor(index),
                };
                match call(&ledgers.handles[index], &recorder, proc, op, "read", vec![], call_timeout).await? {
                    Outcome::Ok(_) => return anyhow::Ok(None),
                    Outcome::Info => proc = procs.next(),
                    Outcome::Fail => {}
                }
                if started.elapsed() > deadline {
                    return Ok(Some(ledger_actor(index)));
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
    }
    let mut unread = Vec::new();
    while let Some(result) = reads.join_next().await {
        unread.extend(result??);
    }
    unread.sort();
    Ok(unread)
}
