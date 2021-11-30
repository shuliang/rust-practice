use crate::client::Client;
use crate::Result;

use bytes::Bytes;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::oneshot;

pub fn buffer(client: Client) -> Buffer {
    let (tx, rx) = channel(32);
    tokio::spawn(async move { run(client, rx).await });
    Buffer { tx }
}

#[derive(Debug)]
enum Command {
    Get(String),
    Set(String, Bytes),
}

type Message = (Command, oneshot::Sender<Result<Option<Bytes>>>);

async fn run(mut client: Client, mut rx: Receiver<Message>) {
    while let Some((cmd, tx)) = rx.recv().await {
        let response = match cmd {
            Command::Get(key) => client.get(&key).await,
            Command::Set(key, value) => client.set(&key, value).await.map(|_| None),
        };
        let _ = tx.send(response);
    }
}

#[derive(Clone)]
pub struct Buffer {
    tx: Sender<Message>,
}

impl Buffer {
    pub async fn get(&mut self, key: &str) -> Result<Option<Bytes>> {
        let get = Command::Get(key.into());
        let (tx, rx) = oneshot::channel();
        self.tx.send((get, tx)).await?;
        match rx.await {
            Ok(res) => res,
            Err(err) => Err(err.into()),
        }
    }

    pub async fn set(&mut self, key: &str, value: Bytes) -> Result<()> {
        let set = Command::Set(key.into(), value);
        let (tx, rx) = oneshot::channel();
        self.tx.send((set, tx)).await?;
        match rx.await {
            Ok(res) => res.map(|_| ()),
            Err(err) => Err(err.into()),
        }
    }
}
