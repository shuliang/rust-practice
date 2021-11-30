use mini_redis::{client, DEFAULT_PORT};

use bytes::Bytes;
use std::num::ParseIntError;
use std::str;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name="mini-redis-cli", version=env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = "Issue Redis commands")]
struct Cli {
    #[structopt(subcommand)]
    command: Command,

    #[structopt(name = "hostname", long = "--host", default_value = "127.0.0.1")]
    host: String,

    #[structopt(name = "port", long = "--port", default_value = DEFAULT_PORT )]
    port: String,
}

#[derive(StructOpt, Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        #[structopt(parse(from_str = bytes_from_str))]
        value: Bytes,
        #[structopt(parse(try_from_str = duration_from_ms_str))]
        expires: Option<Duration>,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> mini_redis::Result<()> {
    tracing_subscriber::fmt::try_init()?;

    let cli = Cli::from_args();
    let addr = format!("{}:{}", cli.host, cli.port);
    let mut client = client::connect(&addr).await?;
    match cli.command {
        Command::Get { key } => {
            if let Some(value) = client.get(&key).await? {
                if let Ok(string) = str::from_utf8(&value) {
                    println!("\"{}\"", string);
                } else {
                    println!("{:?}", value);
                }
            } else {
                println!("(nil)");
            }
        }
        Command::Set {
            key,
            value,
            expires: None,
        } => {
            client.set(&key, value).await?;
            println!("OK");
        }
        Command::Set {
            key,
            value,
            expires: Some(expires),
        } => {
            client.set_expires(&key, value, expires).await?;
            println!("OK");
        }
    }

    Ok(())
}

fn duration_from_ms_str(src: &str) -> Result<Duration, ParseIntError> {
    let ms = src.parse::<u64>()?;
    Ok(Duration::from_millis(ms))
}

fn bytes_from_str(src: &str) -> Bytes {
    Bytes::from(src.to_string())
}
