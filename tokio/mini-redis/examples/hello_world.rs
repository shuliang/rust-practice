//! Hello world server.
//!
//! You can test this out by running:
//!     `cargo run --bin mini-redis-server`
//! And then in another terminal run:
//!     `cargo run --example hello_world`

#![warn(rust_2018_idioms)]

use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    println!("got value from the server; success={:?}", result.is_some());
    Ok(())
}
