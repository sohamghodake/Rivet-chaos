# PR draft: fix the rivetkit-client README

For rivet-dev/rivet, against `rivetkit-rust/packages/client/README.md`. Not opened. Before opening: compile the corrected example against the current crate, and check main again.

---

**Title:** `docs(rivetkit-rust): make the client README example compile on 3.0`

The README's example doesn't compile against `rivetkit-client` 3.0.0, and still says to install 0.1.0:

| README | 3.0.0 crate |
| --- | --- |
| `rivetkit-client = "0.1.0"` | The current version is 3.0.0 |
| `TransportKind::Ws` (example and transport list) | The variant is `TransportKind::WebSocket` (`src/common.rs`) |
| `Client::new(...)?` | `Client::new` returns `Client`, not a `Result` (`src/client.rs`) |
| `["keys-here"].into()` | `ActorKey` is `Vec<String>`, and `[&str; 1].into()` gives a `Vec<&str>` |

The corrected example:

```rust
use rivetkit_client::{Client, ClientConfig, EncodingKind, GetOrCreateOptions, TransportKind};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a client connected to your RivetKit endpoint
    let client = Client::new(
        ClientConfig::new("http://localhost:8080")
            .transport(TransportKind::WebSocket)
            .encoding(EncodingKind::Json),
    );

    // Connect to a chat room actor
    let chat_room = client
        .get_or_create("chat-room", vec!["keys-here".to_string()], GetOrCreateOptions::default())?
        .connect();

    // Listen for new messages
    chat_room
        .on_event("newMessage", |args| {
            let username = args[0].as_str().unwrap();
            let message = args[1].as_str().unwrap();
            println!("Message from {}: {}", username, message);
        })
        .await;

    // Send message to room
    chat_room
        .action("sendMessage", vec![json!("william"), json!("All the world's a stage.")])
        .await?;

    // When finished
    client.disconnect();

    Ok(())
}
```

And in "Supported Transport Methods", `TransportKind::Ws` becomes `TransportKind::WebSocket`.

We hit each of these while writing a harness against the 3.0.0 crate.
