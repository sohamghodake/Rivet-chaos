//! Scripted LLM. A reply depends only on the seed, session and turn, so a rerun
//! with the same seed sees the same conversation, whatever order requests arrive in.
//!
//! Environment: `MOCK_LLM_ADDR`, `MOCK_LLM_SEED`, `MOCK_LLM_DELAY_MS` (as `min,max`).

use std::{env, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use rand::{Rng, SeedableRng, rngs::StdRng};
use serde::{Deserialize, Serialize};

const WORDS: [&str; 8] = [
    "checking", "the", "ledger", "again", "before", "calling", "another", "tool",
];

#[derive(Debug, Clone)]
struct Config {
    seed: u64,
    delay_ms: (u64, u64),
}

#[derive(Debug, Deserialize)]
struct CompleteRequest {
    session: String,
    turn: u64,
}

#[derive(Debug, PartialEq, Serialize)]
struct ToolCall {
    name: String,
}

#[derive(Debug, PartialEq, Serialize)]
struct Completion {
    reply: String,
    tool_calls: Vec<ToolCall>,
}

/// FNV-1a, because `DefaultHasher` output isn't guaranteed across Rust releases.
fn request_seed(seed: u64, session: &str, turn: u64) -> u64 {
    session
        .bytes()
        .chain(turn.to_le_bytes())
        .fold(0xcbf29ce484222325 ^ seed, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
        })
}

fn script(config: &Config, session: &str, turn: u64) -> (Completion, Duration) {
    let mut rng = StdRng::seed_from_u64(request_seed(config.seed, session, turn));
    let reply = (0..rng.random_range(5..=20))
        .map(|_| WORDS[rng.random_range(0..WORDS.len())])
        .collect::<Vec<_>>()
        .join(" ");
    let tool_calls = (0..rng.random_range(1..=3))
        .map(|index| ToolCall {
            name: format!("tool-{index}"),
        })
        .collect();
    let delay = rng.random_range(config.delay_ms.0..=config.delay_ms.1);
    (Completion { reply, tool_calls }, Duration::from_millis(delay))
}

async fn complete(
    State(config): State<Arc<Config>>,
    Json(request): Json<CompleteRequest>,
) -> Json<Completion> {
    let (completion, delay) = script(&config, &request.session, request.turn);
    tokio::time::sleep(delay).await;
    Json(completion)
}

fn parse_delay(value: &str) -> Result<(u64, u64)> {
    let (min, max) = value.split_once(',').context("expected min,max")?;
    let (min, max) = (min.trim().parse()?, max.trim().parse()?);
    anyhow::ensure!(min <= max, "min is greater than max");
    Ok((min, max))
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = env::var("MOCK_LLM_ADDR").unwrap_or_else(|_| "0.0.0.0:6511".into());
    let seed = env::var("MOCK_LLM_SEED").map_or(Ok(42), |value| value.parse())?;
    let delay_ms = env::var("MOCK_LLM_DELAY_MS").map_or(Ok((20, 100)), |value| parse_delay(&value))?;

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/v1/complete", post(complete))
        .with_state(Arc::new(Config { seed, delay_ms }));

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("mock-llm listening on {addr} with seed {seed}");
    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONFIG: Config = Config {
        seed: 42,
        delay_ms: (20, 100),
    };

    #[test]
    fn same_request_gets_the_same_script() {
        assert_eq!(script(&CONFIG, "s1", 3), script(&CONFIG, "s1", 3));
    }

    #[test]
    fn seed_session_and_turn_all_change_the_script() {
        let base = script(&CONFIG, "s1", 3).0;
        let other_seed = Config { seed: 43, ..CONFIG };
        assert_ne!(base, script(&other_seed, "s1", 3).0);
        assert_ne!(base, script(&CONFIG, "s2", 3).0);
        assert_ne!(base, script(&CONFIG, "s1", 4).0);
    }

    #[test]
    fn scripts_stay_in_bounds() {
        for turn in 0..500 {
            let (completion, delay) = script(&CONFIG, "s1", turn);
            assert!((1..=3).contains(&completion.tool_calls.len()));
            assert!((5..=20).contains(&completion.reply.split(' ').count()));
            assert!((20..=100).contains(&delay.as_millis()));
        }
    }

    #[test]
    fn delay_rejects_bad_ranges() {
        assert_eq!(parse_delay("5, 9").unwrap(), (5, 9));
        assert!(parse_delay("9,5").is_err());
        assert!(parse_delay("9").is_err());
    }
}
