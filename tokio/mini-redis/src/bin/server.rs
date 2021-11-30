//! mini-redis server.

use mini_redis::{server, DEFAULT_PORT};

use structopt::StructOpt;
use tokio::{net::TcpListener, signal};

#[tokio::main]
pub async fn main() -> mini_redis::Result<()> {
    tracing_subscriber::fmt::try_init()?;
    let cli = Cli::from_args();
    let port = cli.port.as_deref().unwrap_or(DEFAULT_PORT);
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await?;

    server::run(listener, signal::ctrl_c()).await
}

#[derive(StructOpt, Debug)]
#[structopt(name="mini-redis-server", version=env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = "A Redis server")]
struct Cli {
    #[structopt(name = "port", long = "--port")]
    port: Option<String>,
}
