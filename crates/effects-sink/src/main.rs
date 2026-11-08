//! Records tool calls, alarms and heartbeats outside the cluster, so a fault can't
//! corrupt the evidence used to judge it. Every call gets its own row: a tool call
//! that ran twice shows up as two rows with one key.
//!
//! Rows are stamped with `CLOCK_MONOTONIC`, which the nodes share and which never
//! steps; `GET /clock` lets the harness map it onto its own clock. The wall-clock
//! `received_ms` is kept for reading by eye.
//!
//! `GET /trigger` arms F8: the next tool call to arrive is reported to the waiting
//! caller, which kills that call's node, and the call's own response is held back
//! so the node dies before its workflow step can finish.
//!
//! Environment: `SINK_ADDR`, `SINK_DB`.

use std::{
    env,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

const SCHEMA: &str = "
    PRAGMA journal_mode = WAL;
    CREATE TABLE IF NOT EXISTS effects (
        id INTEGER PRIMARY KEY,
        key TEXT NOT NULL,
        node TEXT NOT NULL,
        received_ms INTEGER NOT NULL,
        mono_ns INTEGER NOT NULL
    );
    CREATE TABLE IF NOT EXISTS alarms (
        id INTEGER PRIMARY KEY,
        key TEXT NOT NULL,
        node TEXT NOT NULL,
        due_ms INTEGER NOT NULL,
        due_mono_ns INTEGER NOT NULL,
        received_ms INTEGER NOT NULL,
        mono_ns INTEGER NOT NULL
    );
    CREATE TABLE IF NOT EXISTS heartbeats (
        id INTEGER PRIMARY KEY,
        key TEXT NOT NULL,
        node TEXT NOT NULL,
        sent_mono_ns INTEGER NOT NULL,
        received_ms INTEGER NOT NULL
    );
";

/// How long a triggering tool call waits for its reply: long enough for the
/// harness to kill the node, and shorter than the node's own 5 s timeout, so a
/// node that isn't killed sees a slow reply rather than a retry.
const TRIGGER_HOLD: Duration = Duration::from_secs(3);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Report {
    key: String,
    node: String,
}

#[derive(Debug, Deserialize)]
struct HeartbeatReport {
    key: String,
    node: String,
    sent_mono_ns: u64,
}

#[derive(Debug, Deserialize)]
struct AlarmReport {
    key: String,
    node: String,
    due_ms: i64,
    due_mono_ns: u64,
}

#[derive(Debug, PartialEq, Serialize)]
struct EffectRow {
    key: String,
    node: String,
    received_ms: i64,
    mono_ns: u64,
}

#[derive(Debug, PartialEq, Serialize)]
struct HeartbeatRow {
    key: String,
    node: String,
    sent_mono_ns: u64,
    received_ms: i64,
}

#[derive(Debug, PartialEq, Serialize)]
struct AlarmRow {
    key: String,
    node: String,
    due_ms: i64,
    due_mono_ns: u64,
    received_ms: i64,
    mono_ns: u64,
}

#[derive(Debug, Deserialize)]
struct Filter {
    #[serde(default)]
    prefix: String,
}

#[derive(Debug, Serialize)]
struct Clock {
    mono_ns: u64,
}

#[derive(Debug, Deserialize)]
struct TriggerWait {
    timeout_s: u64,
}

struct Sink {
    db: Mutex<Connection>,
    trigger: Mutex<Option<oneshot::Sender<Report>>>,
}

/// SQLite has no unsigned integers; nanoseconds of uptime fit in an `i64`.
fn sql_ns(ns: u64) -> i64 {
    ns as i64
}

impl Sink {
    fn open(path: &str) -> rusqlite::Result<Self> {
        let db = Connection::open(path)?;
        db.execute_batch(SCHEMA)?;
        Ok(Self {
            db: Mutex::new(db),
            trigger: Mutex::new(None),
        })
    }

    fn record_effect(&self, report: &Report, now_ms: i64, mono_ns: u64) -> rusqlite::Result<()> {
        self.db.lock().unwrap().execute(
            "INSERT INTO effects (key, node, received_ms, mono_ns) VALUES (?1, ?2, ?3, ?4)",
            params![report.key, report.node, now_ms, sql_ns(mono_ns)],
        )?;
        Ok(())
    }

    fn record_heartbeats(&self, reports: &[HeartbeatReport], now_ms: i64) -> rusqlite::Result<()> {
        let mut db = self.db.lock().unwrap();
        let transaction = db.transaction()?;
        for report in reports {
            transaction.execute(
                "INSERT INTO heartbeats (key, node, sent_mono_ns, received_ms) VALUES (?1, ?2, ?3, ?4)",
                params![report.key, report.node, sql_ns(report.sent_mono_ns), now_ms],
            )?;
        }
        transaction.commit()
    }

    fn record_alarm(
        &self,
        report: &AlarmReport,
        now_ms: i64,
        mono_ns: u64,
    ) -> rusqlite::Result<()> {
        self.db.lock().unwrap().execute(
            "INSERT INTO alarms (key, node, due_ms, due_mono_ns, received_ms, mono_ns)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                report.key,
                report.node,
                report.due_ms,
                sql_ns(report.due_mono_ns),
                now_ms,
                sql_ns(mono_ns)
            ],
        )?;
        Ok(())
    }

    fn effects(&self, prefix: &str) -> rusqlite::Result<Vec<EffectRow>> {
        let db = self.db.lock().unwrap();
        let mut statement = db.prepare(
            "SELECT key, node, received_ms, mono_ns FROM effects
             WHERE substr(key, 1, length(?1)) = ?1 ORDER BY id",
        )?;
        statement
            .query_map([prefix], |row| {
                Ok(EffectRow {
                    key: row.get(0)?,
                    node: row.get(1)?,
                    received_ms: row.get(2)?,
                    mono_ns: row.get::<_, i64>(3)? as u64,
                })
            })?
            .collect()
    }

    fn heartbeats(&self, prefix: &str) -> rusqlite::Result<Vec<HeartbeatRow>> {
        let db = self.db.lock().unwrap();
        let mut statement = db.prepare(
            "SELECT key, node, sent_mono_ns, received_ms FROM heartbeats
             WHERE substr(key, 1, length(?1)) = ?1 ORDER BY id",
        )?;
        statement
            .query_map([prefix], |row| {
                Ok(HeartbeatRow {
                    key: row.get(0)?,
                    node: row.get(1)?,
                    sent_mono_ns: row.get::<_, i64>(2)? as u64,
                    received_ms: row.get(3)?,
                })
            })?
            .collect()
    }

    fn alarms(&self, prefix: &str) -> rusqlite::Result<Vec<AlarmRow>> {
        let db = self.db.lock().unwrap();
        let mut statement = db.prepare(
            "SELECT key, node, due_ms, due_mono_ns, received_ms, mono_ns FROM alarms
             WHERE substr(key, 1, length(?1)) = ?1 ORDER BY id",
        )?;
        statement
            .query_map([prefix], |row| {
                Ok(AlarmRow {
                    key: row.get(0)?,
                    node: row.get(1)?,
                    due_ms: row.get(2)?,
                    due_mono_ns: row.get::<_, i64>(3)? as u64,
                    received_ms: row.get(4)?,
                    mono_ns: row.get::<_, i64>(5)? as u64,
                })
            })?
            .collect()
    }

    /// Arms the trigger, replacing any earlier arming, and returns where the next
    /// tool call will be reported.
    fn arm(&self) -> oneshot::Receiver<Report> {
        let (sender, receiver) = oneshot::channel();
        *self.trigger.lock().unwrap() = Some(sender);
        receiver
    }

    /// Reports a tool call to an armed trigger. True if someone was waiting.
    fn fire(&self, report: &Report) -> bool {
        let sender = self.trigger.lock().unwrap().take();
        sender.is_some_and(|sender| sender.send(report.clone()).is_ok())
    }
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock is before 1970")
        .as_millis() as i64
}

/// The raw `CLOCK_MONOTONIC` reading, the clock Node's `process.hrtime` reads.
fn mono_ns() -> u64 {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // SAFETY: clock_gettime writes the timespec it is given and nothing else.
    let result = unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) };
    assert_eq!(result, 0, "CLOCK_MONOTONIC is always available on Linux");
    ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64
}

type ApiResult<T> = Result<Json<T>, (StatusCode, String)>;

fn internal(error: rusqlite::Error) -> (StatusCode, String) {
    eprintln!("sink database error: {error}");
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}

fn router(sink: Arc<Sink>) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route(
            "/clock",
            get(|| async { Json(Clock { mono_ns: mono_ns() }) }),
        )
        .route(
            "/effects",
            post(
                async |State(sink): State<Arc<Sink>>,
                       Json(report): Json<Report>|
                       -> ApiResult<()> {
                    sink.record_effect(&report, now_ms(), mono_ns())
                        .map_err(internal)?;
                    if sink.fire(&report) {
                        tokio::time::sleep(TRIGGER_HOLD).await;
                    }
                    Ok(Json(()))
                },
            )
            .get(
                async |State(sink): State<Arc<Sink>>,
                       Query(filter): Query<Filter>|
                       -> ApiResult<Vec<EffectRow>> {
                    Ok(Json(sink.effects(&filter.prefix).map_err(internal)?))
                },
            ),
        )
        .route(
            "/heartbeats",
            post(
                async |State(sink): State<Arc<Sink>>,
                       Json(reports): Json<Vec<HeartbeatReport>>|
                       -> ApiResult<()> {
                    sink.record_heartbeats(&reports, now_ms())
                        .map_err(internal)?;
                    Ok(Json(()))
                },
            )
            .get(
                async |State(sink): State<Arc<Sink>>,
                       Query(filter): Query<Filter>|
                       -> ApiResult<Vec<HeartbeatRow>> {
                    Ok(Json(sink.heartbeats(&filter.prefix).map_err(internal)?))
                },
            ),
        )
        .route(
            "/alarms",
            post(
                async |State(sink): State<Arc<Sink>>,
                       Json(report): Json<AlarmReport>|
                       -> ApiResult<()> {
                    sink.record_alarm(&report, now_ms(), mono_ns())
                        .map_err(internal)?;
                    Ok(Json(()))
                },
            )
            .get(
                async |State(sink): State<Arc<Sink>>,
                       Query(filter): Query<Filter>|
                       -> ApiResult<Vec<AlarmRow>> {
                    Ok(Json(sink.alarms(&filter.prefix).map_err(internal)?))
                },
            ),
        )
        .route(
            "/trigger",
            get(
                async |State(sink): State<Arc<Sink>>,
                       Query(wait): Query<TriggerWait>|
                       -> Response {
                    let fired = sink.arm();
                    match tokio::time::timeout(Duration::from_secs(wait.timeout_s), fired).await {
                        Ok(Ok(report)) => Json(report).into_response(),
                        _ => StatusCode::NO_CONTENT.into_response(),
                    }
                },
            ),
        )
        .with_state(sink)
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = env::var("SINK_ADDR").unwrap_or_else(|_| "0.0.0.0:6510".into());
    let path = env::var("SINK_DB").unwrap_or_else(|_| "effects.db".into());
    let sink = Arc::new(Sink::open(&path)?);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("effects-sink listening on {addr}, writing to {path}");
    axum::serve(listener, router(sink)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report(key: &str) -> Report {
        Report {
            key: key.into(),
            node: "node-a".into(),
        }
    }

    #[test]
    fn repeated_effects_stay_visible() {
        let sink = Sink::open(":memory:").unwrap();
        sink.record_effect(&report("s1/1/tool-0"), 10, 100).unwrap();
        sink.record_effect(&report("s1/1/tool-0"), 20, 200).unwrap();

        let rows = sink.effects("").unwrap();
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.key == "s1/1/tool-0"));
        assert_eq!((rows[1].received_ms, rows[1].mono_ns), (20, 200));
    }

    #[test]
    fn prefix_filter_matches_literally() {
        let sink = Sink::open(":memory:").unwrap();
        for key in ["s1/1/tool-0", "s2/1/tool-0", "s%/1/tool-0"] {
            sink.record_effect(&report(key), 10, 10).unwrap();
        }

        assert_eq!(sink.effects("s1/").unwrap().len(), 1);
        assert_eq!(sink.effects("s%/").unwrap().len(), 1);
        assert_eq!(sink.effects("").unwrap().len(), 3);
    }

    #[test]
    fn heartbeat_batches_keep_the_time_each_was_taken() {
        let sink = Sink::open(":memory:").unwrap();
        let beat = |sent_mono_ns| HeartbeatReport {
            key: "3".into(),
            node: "node-b".into(),
            sent_mono_ns,
        };
        sink.record_heartbeats(&[beat(1_000), beat(251_000)], 99)
            .unwrap();

        let rows = sink.heartbeats("3").unwrap();
        assert_eq!(
            rows.iter().map(|row| row.sent_mono_ns).collect::<Vec<_>>(),
            [1_000, 251_000]
        );
        assert!(sink.effects("").unwrap().is_empty());
    }

    #[test]
    fn alarms_keep_their_due_time() {
        let sink = Sink::open(":memory:").unwrap();
        let alarm = AlarmReport {
            key: "s1/1".into(),
            node: "node-a".into(),
            due_ms: 100,
            due_mono_ns: 1_000,
        };
        sink.record_alarm(&alarm, 105, 1_005).unwrap();

        let rows = sink.alarms("s1/").unwrap();
        assert_eq!((rows[0].due_ms, rows[0].due_mono_ns), (100, 1_000));
        assert_eq!((rows[0].received_ms, rows[0].mono_ns), (105, 1_005));
    }

    #[test]
    fn trigger_reports_only_the_next_call_once_armed() {
        let sink = Sink::open(":memory:").unwrap();
        assert!(!sink.fire(&report("s1/1/tool-0")), "nothing armed yet");

        let mut fired = sink.arm();
        assert!(sink.fire(&report("s1/2/tool-1")));
        assert!(!sink.fire(&report("s1/3/tool-0")), "a trigger fires once");
        assert_eq!(fired.try_recv().unwrap().key, "s1/2/tool-1");
    }

    #[test]
    fn rearming_replaces_an_abandoned_wait() {
        let sink = Sink::open(":memory:").unwrap();
        drop(sink.arm());
        let mut fired = sink.arm();
        assert!(sink.fire(&report("s1/1/tool-0")));
        assert_eq!(fired.try_recv().unwrap().node, "node-a");
    }

    #[test]
    fn monotonic_clock_moves_forward() {
        let first = mono_ns();
        assert!(mono_ns() >= first);
    }
}
