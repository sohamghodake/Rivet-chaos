//! Append-only JSON Lines history. The first line is a [`Header`]; every line
//! after it is an [`Event`]. The writer flushes and fsyncs every 100 events, so a
//! crash of the harness loses at most the last few.

use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Read, Write},
    path::Path,
    sync::{Arc, Mutex},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail};
use chaos_core::{Body, Event, Header, Proc, SCHEMA_VERSION};
use flate2::read::GzDecoder;

const SYNC_EVERY: usize = 100;

/// A whole history, as read back for checking.
#[derive(Debug, Clone)]
pub struct History {
    pub header: Header,
    pub events: Vec<Event>,
}

struct Writer {
    out: BufWriter<File>,
    run: String,
    seq: u64,
    started: Instant,
    unsynced: usize,
}

impl Writer {
    fn sync(&mut self) -> io::Result<()> {
        self.out.flush()?;
        self.out.get_ref().sync_data()?;
        self.unsynced = 0;
        Ok(())
    }
}

/// Records events from any task. The sequence number and timestamp are taken
/// under the same lock as the write, so `seq` order is also time order.
#[derive(Clone)]
pub struct Recorder {
    writer: Arc<Mutex<Writer>>,
}

impl Recorder {
    pub fn create(path: &Path, header: &Header) -> Result<Self> {
        let file = File::create(path).with_context(|| format!("create {}", path.display()))?;
        let mut out = BufWriter::new(file);
        serde_json::to_writer(&mut out, header)?;
        out.write_all(b"\n")?;
        let mut writer = Writer {
            out,
            run: header.run.clone(),
            seq: 0,
            started: Instant::now(),
            unsynced: 0,
        };
        writer.sync()?;
        Ok(Self {
            writer: Arc::new(Mutex::new(writer)),
        })
    }

    /// Writes one event and returns its timestamp in nanoseconds since the
    /// recorder was created. `wall` also stamps the wall clock.
    pub fn record(&self, proc: Proc, body: Body, wall: bool) -> Result<u64> {
        let mut writer = self.writer.lock().unwrap();
        let mono_ns = writer.started.elapsed().as_nanos() as u64;
        let wall_ms = wall.then(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock is before 1970")
                .as_millis() as u64
        });
        writer.seq += 1;
        let event = Event {
            run: writer.run.clone(),
            seq: writer.seq,
            proc,
            body,
            mono_ns,
            wall_ms,
        };
        serde_json::to_writer(&mut writer.out, &event)?;
        writer.out.write_all(b"\n")?;
        writer.unsynced += 1;
        if writer.unsynced >= SYNC_EVERY {
            writer.sync()?;
        }
        Ok(mono_ns)
    }

    /// Nanoseconds since the recorder was created, on the same clock as events.
    pub fn now_ns(&self) -> u64 {
        self.writer.lock().unwrap().started.elapsed().as_nanos() as u64
    }

    pub fn close(&self) -> Result<()> {
        self.writer.lock().unwrap().sync()?;
        Ok(())
    }
}

/// Reads a history written by [`Recorder`], plain or gzipped (`.gz`).
pub fn read(path: &Path) -> Result<History> {
    let file = File::open(path).with_context(|| format!("open {}", path.display()))?;
    let input: Box<dyn Read> = if path.extension().is_some_and(|ext| ext == "gz") {
        Box::new(GzDecoder::new(file))
    } else {
        Box::new(file)
    };
    parse(BufReader::new(input)).with_context(|| format!("read {}", path.display()))
}

pub fn parse(input: impl BufRead) -> Result<History> {
    let mut lines = input.lines().enumerate();
    let (_, first) = lines.next().context("history is empty")?;
    let header: Header = serde_json::from_str(&first?).context("line 1: bad header")?;
    if header.schema != SCHEMA_VERSION {
        bail!(
            "history uses schema {}, this build reads schema {SCHEMA_VERSION}",
            header.schema
        );
    }

    let mut events = Vec::new();
    for (index, line) in lines {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let event: Event =
            serde_json::from_str(&line).with_context(|| format!("line {}", index + 1))?;
        events.push(event);
    }
    Ok(History { header, events })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use chaos_core::Op;

    use super::*;

    fn header() -> Header {
        Header {
            schema: SCHEMA_VERSION,
            run: "test-run".into(),
            scenario: "test".into(),
            seed: 7,
            started_wall_ms: 0,
            scenario_toml: String::new(),
            versions: BTreeMap::new(),
            host: BTreeMap::new(),
        }
    }

    fn invoke(n: i64) -> Body {
        Body::Invoke {
            op: Op::Append {
                actor: "ledger/0".into(),
                op_id: format!("p0-{n:06}"),
                value: n,
            },
        }
    }

    fn temp_path(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("chaos-history-{}-{name}", std::process::id()))
    }

    #[test]
    fn events_read_back_in_order_with_rising_timestamps() {
        let path = temp_path("order.jsonl");
        let recorder = Recorder::create(&path, &header()).unwrap();
        for n in 0..250 {
            recorder.record(Proc::Client(0), invoke(n), n == 0).unwrap();
        }
        recorder.close().unwrap();

        let history = read(&path).unwrap();
        std::fs::remove_file(&path).unwrap();
        assert_eq!(history.header, header());
        assert_eq!(history.events.len(), 250);
        assert!(
            history
                .events
                .windows(2)
                .all(|pair| pair[0].seq + 1 == pair[1].seq && pair[0].mono_ns <= pair[1].mono_ns)
        );
        assert!(history.events[0].wall_ms.is_some());
        assert!(history.events[1].wall_ms.is_none());
    }

    #[test]
    fn events_are_on_disk_before_close_after_a_sync() {
        let path = temp_path("sync.jsonl");
        let recorder = Recorder::create(&path, &header()).unwrap();
        for n in 0..SYNC_EVERY as i64 {
            recorder.record(Proc::Client(0), invoke(n), false).unwrap();
        }
        // No close: a crash here should still leave the first 100 readable.
        let history = read(&path).unwrap();
        std::fs::remove_file(&path).unwrap();
        assert_eq!(history.events.len(), SYNC_EVERY);
    }

    #[test]
    fn other_schema_versions_are_rejected() {
        let mut old = header();
        old.schema = SCHEMA_VERSION + 1;
        let text = serde_json::to_string(&old).unwrap();
        let error = parse(text.as_bytes()).unwrap_err().to_string();
        assert!(error.contains("schema"), "{error}");
    }

    #[test]
    fn gzipped_histories_read_the_same() {
        use flate2::{Compression, write::GzEncoder};

        let path = temp_path("plain.jsonl");
        let recorder = Recorder::create(&path, &header()).unwrap();
        recorder.record(Proc::Nemesis, invoke(1), true).unwrap();
        recorder.close().unwrap();

        let gz_path = temp_path("zipped.jsonl.gz");
        let mut encoder = GzEncoder::new(File::create(&gz_path).unwrap(), Compression::default());
        encoder.write_all(&std::fs::read(&path).unwrap()).unwrap();
        encoder.finish().unwrap();

        let plain = read(&path).unwrap();
        let zipped = read(&gz_path).unwrap();
        std::fs::remove_file(&path).unwrap();
        std::fs::remove_file(&gz_path).unwrap();
        assert_eq!(plain.events, zipped.events);
    }
}
