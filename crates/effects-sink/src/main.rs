//! Records tool calls, alarms and heartbeats outside the cluster, so a fault can't
//! corrupt the evidence used to judge it. Every call gets its own row: a tool call
//! that ran twice shows up as two rows with one key.
//!
//! Environment: `SINK_ADDR`, `SINK_DB`.

use std::{
    env,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

const SCHEMA: &str = "
    PRAGMA journal_mode = WAL;
    CREATE TABLE IF NOT EXISTS effects (
        id INTEGER PRIMARY KEY,
        key TEXT NOT NULL,
        node TEXT NOT NULL,
        received_ms INTEGER NOT NULL
    );
    CREATE TABLE IF NOT EXISTS alarms (
        id INTEGER PRIMARY KEY,
        key TEXT NOT NULL,
        node TEXT NOT NULL,
        due_ms INTEGER NOT NULL,
        received_ms INTEGER NOT NULL
    );
    CREATE TABLE IF NOT EXISTS heartbeats (
        id INTEGER PRIMARY KEY,
        key TEXT NOT NULL,
        node TEXT NOT NULL,
        received_ms INTEGER NOT NULL
    );
";

#[derive(Debug, Deserialize)]
struct Report {
    key: String,
    node: String,
}

#[derive(Debug, Deserialize)]
struct AlarmReport {
    key: String,
    node: String,
    due_ms: i64,
}

#[derive(Debug, PartialEq, Serialize)]
struct Row {
    key: String,
    node: String,
    received_ms: i64,
}

#[derive(Debug, PartialEq, Serialize)]
struct AlarmRow {
    key: String,
    node: String,
    due_ms: i64,
    received_ms: i64,
}

#[derive(Debug, Deserialize)]
struct Filter {
    #[serde(default)]
    prefix: String,
}

struct Sink {
    db: Mutex<Connection>,
}

impl Sink {
    fn open(path: &str) -> rusqlite::Result<Self> {
        let db = Connection::open(path)?;
        db.execute_batch(SCHEMA)?;
        Ok(Self { db: Mutex::new(db) })
    }

    fn record(&self, table: &str, report: &Report, now_ms: i64) -> rusqlite::Result<()> {
        self.db.lock().unwrap().execute(
            &format!("INSERT INTO {table} (key, node, received_ms) VALUES (?1, ?2, ?3)"),
            params![report.key, report.node, now_ms],
        )?;
        Ok(())
    }

    fn record_alarm(&self, report: &AlarmReport, now_ms: i64) -> rusqlite::Result<()> {
        self.db.lock().unwrap().execute(
            "INSERT INTO alarms (key, node, due_ms, received_ms) VALUES (?1, ?2, ?3, ?4)",
            params![report.key, report.node, report.due_ms, now_ms],
        )?;
        Ok(())
    }

    fn rows(&self, table: &str, prefix: &str) -> rusqlite::Result<Vec<Row>> {
        let db = self.db.lock().unwrap();
        let mut statement = db.prepare(&format!(
            "SELECT key, node, received_ms FROM {table}
             WHERE substr(key, 1, length(?1)) = ?1 ORDER BY id"
        ))?;
        statement
            .query_map([prefix], |row| {
                Ok(Row {
                    key: row.get(0)?,
                    node: row.get(1)?,
                    received_ms: row.get(2)?,
                })
            })?
            .collect()
    }

    fn alarm_rows(&self, prefix: &str) -> rusqlite::Result<Vec<AlarmRow>> {
        let db = self.db.lock().unwrap();
        let mut statement = db.prepare(
            "SELECT key, node, due_ms, received_ms FROM alarms
             WHERE substr(key, 1, length(?1)) = ?1 ORDER BY id",
        )?;
        statement
            .query_map([prefix], |row| {
                Ok(AlarmRow {
                    key: row.get(0)?,
                    node: row.get(1)?,
                    due_ms: row.get(2)?,
                    received_ms: row.get(3)?,
                })
            })?
            .collect()
    }
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock is before 1970")
        .as_millis() as i64
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
            "/effects",
            post(
                async |State(sink): State<Arc<Sink>>, Json(report): Json<Report>| -> ApiResult<()> {
                    sink.record("effects", &report, now_ms()).map_err(internal)?;
                    Ok(Json(()))
                },
            )
            .get(
                async |State(sink): State<Arc<Sink>>, Query(filter): Query<Filter>| -> ApiResult<Vec<Row>> {
                    Ok(Json(sink.rows("effects", &filter.prefix).map_err(internal)?))
                },
            ),
        )
        .route(
            "/heartbeats",
            post(
                async |State(sink): State<Arc<Sink>>, Json(report): Json<Report>| -> ApiResult<()> {
                    sink.record("heartbeats", &report, now_ms()).map_err(internal)?;
                    Ok(Json(()))
                },
            )
            .get(
                async |State(sink): State<Arc<Sink>>, Query(filter): Query<Filter>| -> ApiResult<Vec<Row>> {
                    Ok(Json(sink.rows("heartbeats", &filter.prefix).map_err(internal)?))
                },
            ),
        )
        .route(
            "/alarms",
            post(
                async |State(sink): State<Arc<Sink>>, Json(report): Json<AlarmReport>| -> ApiResult<()> {
                    sink.record_alarm(&report, now_ms()).map_err(internal)?;
                    Ok(Json(()))
                },
            )
            .get(
                async |State(sink): State<Arc<Sink>>, Query(filter): Query<Filter>| -> ApiResult<Vec<AlarmRow>> {
                    Ok(Json(sink.alarm_rows(&filter.prefix).map_err(internal)?))
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
        sink.record("effects", &report("s1/1/tool-0"), 10).unwrap();
        sink.record("effects", &report("s1/1/tool-0"), 20).unwrap();

        let rows = sink.rows("effects", "").unwrap();
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.key == "s1/1/tool-0"));
        assert_eq!(rows[1].received_ms, 20);
    }

    #[test]
    fn prefix_filter_matches_literally() {
        let sink = Sink::open(":memory:").unwrap();
        sink.record("effects", &report("s1/1/tool-0"), 10).unwrap();
        sink.record("effects", &report("s2/1/tool-0"), 10).unwrap();
        sink.record("effects", &report("s%/1/tool-0"), 10).unwrap();

        assert_eq!(sink.rows("effects", "s1/").unwrap().len(), 1);
        assert_eq!(sink.rows("effects", "s%/").unwrap().len(), 1);
        assert_eq!(sink.rows("effects", "").unwrap().len(), 3);
    }

    #[test]
    fn tables_are_kept_separate() {
        let sink = Sink::open(":memory:").unwrap();
        sink.record("heartbeats", &report("s1"), 10).unwrap();
        assert!(sink.rows("effects", "").unwrap().is_empty());
        assert_eq!(sink.rows("heartbeats", "").unwrap().len(), 1);
    }

    #[test]
    fn alarms_keep_their_due_time() {
        let sink = Sink::open(":memory:").unwrap();
        let alarm = AlarmReport {
            key: "s1/1".into(),
            node: "node-a".into(),
            due_ms: 100,
        };
        sink.record_alarm(&alarm, 105).unwrap();

        let rows = sink.alarm_rows("s1/").unwrap();
        assert_eq!(rows[0].due_ms, 100);
        assert_eq!(rows[0].received_ms, 105);
    }
}
