//! Subscribe to a redis channel example.
//!
//! You can test this out by running:
//!     `cargo run --bin mini-redis-server`
//! Then in another terminal run:
//!     `cargo run --example sub`
//! And then in another terminal run:
//!     `cargo run --example pub`

#![warn(rust_2018_idioms)]

use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;

    let mut subscriber = client.subscribe(vec!["foo".into()]).await?;

    if let Some(msg) = subscriber.next_message().await? {
        println!(
            "got message from the channel: {}; message = {:?}",
            msg.channel, msg.content
        );
    }

    Ok(())
}
