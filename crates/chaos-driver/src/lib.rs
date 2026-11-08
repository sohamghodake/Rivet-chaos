//! Logical client processes that call actor actions with pacing and timeouts.
//!
//! Each process makes one call at a time and records it as an `invoke` followed
//! by exactly one result. A process whose call ends in `info` retires, and its
//! slot carries on under a new process ID, so the order of calls within any one
//! process stays meaningful.

use std::{
    future::Future,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    time::{Duration, Instant},
};

use anyhow::{Context, Result, bail};
use chaos_core::{Body, Op, Proc, Turn};
use chaos_history::Recorder;
use rand::{Rng, SeedableRng, rngs::StdRng};
use rivetkit_client::{
    Client, GetOrCreateOptions,
    handle::{ActorHandle, SendOpts},
};
use serde_json::{Value, json};
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

pub fn ledger_actor(index: usize) -> String {
    format!("ledger/{index}")
}

pub fn session_actor(index: usize) -> String {
    format!("session/{index}")
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

fn handles(client: &Client, name: &str, count: u32) -> Result<Arc<Vec<ActorHandle>>> {
    let handles = (0..count)
        .map(|index| {
            client.get_or_create(name, vec![index.to_string()], GetOrCreateOptions::default())
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Arc::new(handles))
}

#[derive(Clone)]
pub struct Ledgers {
    handles: Arc<Vec<ActorHandle>>,
}

impl Ledgers {
    pub fn new(client: &Client, count: u32) -> Result<Self> {
        Ok(Self {
            handles: handles(client, "ledger", count)?,
        })
    }

    pub fn len(&self) -> usize {
        self.handles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.handles.is_empty()
    }
}

#[derive(Clone)]
pub struct Sessions {
    handles: Arc<Vec<ActorHandle>>,
}

impl Sessions {
    pub fn new(client: &Client, count: u32) -> Result<Self> {
        Ok(Self {
            handles: handles(client, "agent-session", count)?,
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
    error.chain().any(|cause| {
        cause
            .downcast_ref::<reqwest::Error>()
            .is_some_and(reqwest::Error::is_connect)
    })
}

/// An error as the history keeps it, without the admin token request URLs carry.
fn describe(error: &anyhow::Error) -> String {
    let text = format!("{error:#}");
    match text.find("rvt-token=") {
        None => text,
        Some(start) => {
            let value = start + "rvt-token=".len();
            let end = text[value..]
                .find(|c: char| !c.is_ascii_alphanumeric())
                .map_or(text.len(), |end| value + end);
            format!("{}rvt-token=…{}", &text[..start], &text[end..])
        }
    }
}

/// Records `op`, runs `request` with a timeout, and records how it ended.
/// Nothing is sent until `request` is first polled, after the invoke is written.
async fn call(
    recorder: &Recorder,
    proc: Proc,
    op: Op,
    timeout: Duration,
    request: impl Future<Output = Result<Value>>,
) -> Result<Outcome> {
    recorder.record(proc, Body::Invoke { op: op.clone() }, true)?;
    let (body, outcome) = match tokio::time::timeout(timeout, request).await {
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
                error: describe(&error),
            },
            Outcome::Fail,
        ),
        Ok(Err(error)) => (
            Body::Info {
                op,
                error: describe(&error),
            },
            Outcome::Info,
        ),
        Ok(Ok(value)) => {
            let (op_ids, turns) = match &op {
                Op::Read { .. } => (
                    Some(serde_json::from_value(value.clone()).context("read result")?),
                    None,
                ),
                Op::Turns { .. } => (
                    None,
                    Some(serde_json::from_value(value.clone()).context("turns result")?),
                ),
                Op::Append { .. } | Op::Send { .. } => (None, None),
            };
            let body = Body::Ok {
                duplicate: matches!(op, Op::Append { .. }) && value == "duplicate",
                op,
                op_ids,
                turns,
            };
            (body, Outcome::Ok(value))
        }
    };
    recorder.record(proc, body, false)?;
    Ok(outcome)
}

/// Calls every actor until each has answered once, so a run doesn't open with
/// the misses of a cluster that is still settling. Nothing is recorded.
pub async fn warm_up(
    ledgers: &Ledgers,
    sessions: &Sessions,
    call_timeout: Duration,
    deadline: Duration,
) -> Result<()> {
    let started = Instant::now();
    let mut calls = JoinSet::new();
    let actors = (0..ledgers.len())
        .map(|index| (ledger_actor(index), ledgers.handles.clone(), index, "read"))
        .chain((0..sessions.len()).map(|index| {
            (
                session_actor(index),
                sessions.handles.clone(),
                index,
                "turns",
            )
        }));
    for (actor, handles, index, action) in actors {
        calls.spawn(async move {
            loop {
                if let Ok(Ok(_)) =
                    tokio::time::timeout(call_timeout, handles[index].action(action, vec![])).await
                {
                    return Ok(());
                }
                if started.elapsed() > deadline {
                    bail!("{actor} didn't answer within {deadline:?} of the cluster starting");
                }
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        });
    }
    while let Some(result) = calls.join_next().await {
        result??;
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct LoadConfig {
    pub processes: u32,
    pub pacing_ms: [u64; 2],
    pub read_percent: u32,
    pub call_timeout: Duration,
    pub seed: u64,
}

/// Calls random ledgers from `processes` slots until `stop` is cancelled, then
/// waits for the calls in flight to finish. Most calls append; `read_percent` of
/// them read, for I4.
pub async fn run_load(
    ledgers: Ledgers,
    recorder: Recorder,
    procs: Procs,
    config: LoadConfig,
    stop: CancellationToken,
) -> Result<()> {
    let mut slots = JoinSet::new();
    for slot in 0..config.processes {
        let (ledgers, recorder, procs, config, stop) = (
            ledgers.clone(),
            recorder.clone(),
            procs.clone(),
            config.clone(),
            stop.clone(),
        );
        slots.spawn(async move {
            // Each slot gets its own stream of choices from the run's seed.
            let slot_seed = config.seed ^ (u64::from(slot) + 1).wrapping_mul(0x9e37_79b9_7f4a_7c15);
            let mut rng = StdRng::seed_from_u64(slot_seed);
            let mut proc = procs.next();
            let mut count = 0u32;
            while !stop.is_cancelled() {
                let index = rng.random_range(0..ledgers.len());
                let handle = &ledgers.handles[index];
                let actor = ledger_actor(index);
                let outcome = if rng.random_range(0..100) < config.read_percent {
                    let op = Op::Read { actor };
                    call(
                        &recorder,
                        proc,
                        op,
                        config.call_timeout,
                        handle.action("read", vec![]),
                    )
                    .await?
                } else {
                    let Proc::Client(id) = proc else {
                        unreachable!()
                    };
                    let op_id = format!("p{id}-{count:06}");
                    let args = vec![json!(op_id), json!(count)];
                    let op = Op::Append {
                        actor,
                        op_id,
                        value: i64::from(count),
                    };
                    count += 1;
                    call(
                        &recorder,
                        proc,
                        op,
                        config.call_timeout,
                        handle.action("append", args),
                    )
                    .await?
                };
                if outcome == Outcome::Info {
                    proc = procs.next();
                    count = 0;
                }
                let pause = Duration::from_millis(
                    rng.random_range(config.pacing_ms[0]..=config.pacing_ms[1]),
                );
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

#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub turn_pacing_ms: [u64; 2],
    pub call_timeout: Duration,
    pub seed: u64,
}

/// Sends each session a new turn every `turn_pacing_ms` until `stop` is
/// cancelled. Returns, per session, the turns whose send was acknowledged. A turn
/// number is never reused, because a send that ended in `info` may still arrive.
pub async fn run_sessions(
    sessions: Sessions,
    recorder: Recorder,
    procs: Procs,
    config: SessionConfig,
    stop: CancellationToken,
) -> Result<Vec<Vec<u32>>> {
    let mut tasks = JoinSet::new();
    for index in 0..sessions.len() {
        let (sessions, recorder, procs, config, stop) = (
            sessions.clone(),
            recorder.clone(),
            procs.clone(),
            config.clone(),
            stop.clone(),
        );
        tasks.spawn(async move {
            let session_seed =
                config.seed ^ 0x7365_7373 ^ (index as u64 + 1).wrapping_mul(0x9e37_79b9_7f4a_7c15);
            let mut rng = StdRng::seed_from_u64(session_seed);
            let handle = &sessions.handles[index];
            let mut proc = procs.next();
            let mut acknowledged = Vec::new();
            let mut turn = 1u32;
            while !stop.is_cancelled() {
                let op = Op::Send {
                    actor: session_actor(index),
                    turn,
                };
                let message = json!({ "turn": turn, "text": format!("message {turn}") });
                let request = async {
                    handle
                        .send("message", message, SendOpts::default())
                        .await
                        .map(|()| Value::Null)
                };
                match call(&recorder, proc, op, config.call_timeout, request).await? {
                    Outcome::Ok(_) => acknowledged.push(turn),
                    Outcome::Info => proc = procs.next(),
                    Outcome::Fail => {}
                }
                turn += 1;
                let pause = Duration::from_millis(
                    rng.random_range(config.turn_pacing_ms[0]..=config.turn_pacing_ms[1]),
                );
                tokio::select! {
                    _ = stop.cancelled() => break,
                    _ = tokio::time::sleep(pause) => {}
                }
            }
            anyhow::Ok((index, acknowledged))
        });
    }
    let mut acknowledged = vec![Vec::new(); sessions.len()];
    while let Some(result) = tasks.join_next().await {
        let (index, turns) = result??;
        acknowledged[index] = turns;
    }
    Ok(acknowledged)
}

/// Polls each session's finished turns until every acknowledged turn is there or
/// `deadline` passes. Returns the sessions that didn't finish.
pub async fn drain_sessions(
    sessions: Sessions,
    recorder: Recorder,
    procs: Procs,
    acknowledged: Vec<Vec<u32>>,
    call_timeout: Duration,
    deadline: Duration,
) -> Result<Vec<String>> {
    let started = Instant::now();
    let mut polls = JoinSet::new();
    for (index, expected) in acknowledged.into_iter().enumerate() {
        let (sessions, recorder, procs) = (sessions.clone(), recorder.clone(), procs.clone());
        polls.spawn(async move {
            let handle = &sessions.handles[index];
            let mut proc = procs.next();
            loop {
                let op = Op::Turns {
                    actor: session_actor(index),
                };
                match call(
                    &recorder,
                    proc,
                    op,
                    call_timeout,
                    handle.action("turns", vec![]),
                )
                .await?
                {
                    Outcome::Ok(value) => {
                        let turns: Vec<Turn> = serde_json::from_value(value)?;
                        if expected
                            .iter()
                            .all(|turn| turns.iter().any(|done| done.turn == *turn))
                        {
                            return anyhow::Ok(None);
                        }
                    }
                    Outcome::Info => proc = procs.next(),
                    Outcome::Fail => {}
                }
                if started.elapsed() > deadline {
                    return Ok(Some(session_actor(index)));
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
    }
    let mut unfinished = Vec::new();
    while let Some(result) = polls.join_next().await {
        unfinished.extend(result??);
    }
    unfinished.sort();
    Ok(unfinished)
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
            let handle = &ledgers.handles[index];
            let mut proc = procs.next();
            loop {
                let op = Op::Read {
                    actor: ledger_actor(index),
                };
                match call(
                    &recorder,
                    proc,
                    op,
                    call_timeout,
                    handle.action("read", vec![]),
                )
                .await?
                {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_lose_the_token() {
        let error = anyhow::anyhow!(
            "error sending request for url (http://h/gateway?rvt-key=2&rvt-token=d5515e03&x=1)"
        );
        assert_eq!(
            describe(&error),
            "error sending request for url (http://h/gateway?rvt-key=2&rvt-token=…&x=1)"
        );
        assert_eq!(describe(&anyhow::anyhow!("timed out")), "timed out");
    }
}
