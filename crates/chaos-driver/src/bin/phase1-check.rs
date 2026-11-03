//! Checks phase 1's exit criteria against a running engine, node and sink: 1,000
//! appends read back exactly, one session completes 10 turns, and the sink shows
//! one effect per tool step.
//!
//! Environment: `RIVET_ENDPOINT` (required), `SINK_URL`.

use std::{
    collections::{BTreeMap, HashSet},
    env,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail, ensure};
use rivetkit_client::{
    Client, ClientConfig, EncodingKind, GetOrCreateOptions, TransportKind, handle::SendOpts,
};
use serde::Deserialize;
use serde_json::json;

const APPENDS: usize = 1_000;
const TURNS: usize = 10;
const SESSION_DEADLINE: Duration = Duration::from_secs(120);

#[derive(Debug, Deserialize)]
struct Turn {
    turn: usize,
    reply: String,
    tool_calls: usize,
}

#[derive(Debug, Deserialize)]
struct Effect {
    key: String,
}

#[derive(Debug, Deserialize)]
struct Alarm {
    key: String,
    due_ms: i64,
    received_ms: i64,
}

#[derive(Debug, Deserialize)]
struct Heartbeat {
    node: String,
}

fn percentile(sorted: &[Duration], percent: usize) -> Duration {
    sorted[(sorted.len() - 1) * percent / 100]
}

async fn check_ledger(client: &Client, run: &str) -> Result<()> {
    let ledger = client.get_or_create("ledger", vec![run.into()], GetOrCreateOptions::default())?;

    let started = Instant::now();
    let mut latencies = Vec::with_capacity(APPENDS);
    let mut expected = HashSet::with_capacity(APPENDS);
    for value in 0..APPENDS {
        let op_id = format!("p0-{value:06}");
        let call = Instant::now();
        let result = ledger
            .action("append", vec![json!(op_id), json!(value)])
            .await
            .with_context(|| format!("append {op_id}"))?;
        latencies.push(call.elapsed());
        ensure!(result == "inserted", "append {op_id} returned {result}");
        expected.insert(op_id);
    }
    latencies.sort();
    println!(
        "ledger: {APPENDS} appends in {:.1?}, p50 {:.1?}, p99 {:.1?}",
        started.elapsed(),
        percentile(&latencies, 50),
        percentile(&latencies, 99),
    );

    let repeat = ledger
        .action("append", vec![json!("p0-000000"), json!(0)])
        .await?;
    ensure!(repeat == "duplicate", "repeated op_id returned {repeat}");

    let read = ledger.action("read", vec![]).await?;
    let read: Vec<String> = serde_json::from_value(read)?;
    let found: HashSet<String> = read.iter().cloned().collect();
    ensure!(read.len() == APPENDS, "read {} rows, expected {APPENDS}", read.len());
    ensure!(found == expected, "read-back IDs differ from the IDs appended");
    println!("ledger: read back exactly {APPENDS} entries, repeated op_id reported as duplicate");
    Ok(())
}

async fn get_json<T: for<'de> Deserialize<'de>>(sink: &str, path: &str, prefix: &str) -> Result<T> {
    Ok(reqwest::Client::new()
        .get(format!("{sink}/{path}"))
        .query(&[("prefix", prefix)])
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?)
}

async fn check_session(client: &Client, sink: &str, run: &str) -> Result<()> {
    let session = client.get_or_create(
        "agent-session",
        vec![run.into()],
        GetOrCreateOptions::default(),
    )?;

    let connection = session.connect();
    let tokens = Arc::new(AtomicUsize::new(0));
    let counted = tokens.clone();
    let _subscription = connection
        .on_event("token", move |_args| {
            counted.fetch_add(1, Ordering::Relaxed);
        })
        .await;

    for turn in 1..=TURNS {
        session
            .send(
                "message",
                json!({ "turn": turn, "text": format!("message {turn}") }),
                SendOpts::default(),
            )
            .await
            .with_context(|| format!("queue turn {turn}"))?;
    }

    let deadline = Instant::now() + SESSION_DEADLINE;
    let turns: Vec<Turn> = loop {
        let turns: Vec<Turn> = serde_json::from_value(session.action("turns", vec![]).await?)?;
        if turns.len() == TURNS {
            break turns;
        }
        if Instant::now() > deadline {
            bail!("{} of {TURNS} turns finished before the deadline", turns.len());
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    };
    let numbers: Vec<usize> = turns.iter().map(|turn| turn.turn).collect();
    ensure!(numbers == (1..=TURNS).collect::<Vec<_>>(), "turns were {numbers:?}");
    println!("session: {TURNS} turns completed");

    let prefix = format!("{run}/");
    let effects: Vec<Effect> = get_json(sink, "effects", &prefix).await?;
    let mut calls: BTreeMap<String, usize> = BTreeMap::new();
    for effect in &effects {
        *calls.entry(effect.key.clone()).or_default() += 1;
    }
    let expected: BTreeMap<String, usize> = turns
        .iter()
        .flat_map(|turn| {
            (0..turn.tool_calls).map(|index| (format!("{run}/{}/tool-{index}", turn.turn), 1))
        })
        .collect();
    ensure!(
        calls == expected,
        "sink effects don't match tool steps: got {calls:?}, expected {expected:?}"
    );
    println!("sink: exactly one effect for each of {} tool steps", expected.len());

    // Alarms fire about a second after their turn, so give the last ones time to land.
    let deadline = Instant::now() + Duration::from_secs(30);
    let alarms: Vec<Alarm> = loop {
        let alarms: Vec<Alarm> = get_json(sink, "alarms", &prefix).await?;
        if alarms.len() >= TURNS || Instant::now() > deadline {
            break alarms;
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    };
    let keys: HashSet<&str> = alarms.iter().map(|alarm| alarm.key.as_str()).collect();
    ensure!(
        alarms.len() == TURNS && keys.len() == TURNS,
        "expected one alarm per turn, got {} alarms for {} turns",
        alarms.len(),
        keys.len()
    );
    let early: Vec<&Alarm> = alarms.iter().filter(|alarm| alarm.received_ms < alarm.due_ms).collect();
    ensure!(early.is_empty(), "alarms fired early: {early:?}");
    println!("sink: one alarm per turn, none early");

    let words: usize = turns.iter().map(|turn| turn.reply.split(' ').count()).sum();
    let deadline = Instant::now() + Duration::from_secs(5);
    while tokens.load(Ordering::Relaxed) < words && Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    let received = tokens.load(Ordering::Relaxed);
    ensure!(received == words, "received {received} broadcast tokens, expected {words}");
    println!("session: received all {words} broadcast tokens over WebSocket");

    let heartbeats: Vec<Heartbeat> = get_json(sink, "heartbeats", run).await?;
    let nodes: HashSet<&str> = heartbeats.iter().map(|beat| beat.node.as_str()).collect();
    ensure!(!heartbeats.is_empty(), "the sink saw no heartbeats");
    println!("sink: {} heartbeats from nodes {nodes:?}", heartbeats.len());

    connection.disconnect().await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = env::var("RIVET_ENDPOINT").context("RIVET_ENDPOINT is required")?;
    let sink = env::var("SINK_URL").unwrap_or_else(|_| "http://localhost:6510".into());
    let started = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let run = format!("check-{started}");

    let client = Client::new(
        ClientConfig::new(endpoint)
            .transport(TransportKind::WebSocket)
            .encoding(EncodingKind::Json),
    );

    check_ledger(&client, &run).await?;
    check_session(&client, &sink, &run).await?;
    println!("phase 1 exit criteria passed ({run})");
    Ok(())
}
