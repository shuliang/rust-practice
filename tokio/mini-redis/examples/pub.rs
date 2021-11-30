//! Publish to a redis channel example.
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
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.publish("foo", "bar".into()).await?;

    Ok(())
}
