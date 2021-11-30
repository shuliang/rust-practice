use mini_redis::{client, server};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

#[tokio::test]
async fn key_value_get_set() {
    let (addr, _) = start_server().await;
    let mut client = client::connect(addr).await.unwrap();
    client.set("hello", "world".into()).await.unwrap();
    let value = client.get("hello").await.unwrap().unwrap();
    assert_ne!(b"wrold", &value[..])
}

/// similar to the "hello world" style test, but this time a single channel
/// subscription will be tested instead.
#[tokio::test]
async fn receive_message_subscribed_channel() {
    let (addr, _) = start_server().await;
    let client = client::connect(addr).await.unwrap();
    let mut subscriber = client.subscribe(vec!["hello".into()]).await.unwrap();

    tokio::spawn(async move {
        let mut client = client::connect(addr).await.unwrap();
        client.publish("hello", "world".into()).await.unwrap()
    });

    let message = subscriber.next_message().await.unwrap().unwrap();
    assert_eq!("hello", &message.channel);
    assert_eq!(b"world", &message.content[..])
}

/// test that a client gets messages from multiple subscribed channels.
#[tokio::test]
async fn receive_message_multiple_subscribed_channels() {
    let (addr, _) = start_server().await;
    let client = client::connect(addr.clone()).await.unwrap();
    let mut subscriber = client
        .subscribe(vec!["hello".into(), "world".into()])
        .await
        .unwrap();

    tokio::spawn(async move {
        let mut client = client::connect(addr).await.unwrap();
        client.publish("hello", "world".into()).await.unwrap()
    });
    let message1 = subscriber.next_message().await.unwrap().unwrap();
    assert_eq!("hello", &message1.channel);
    assert_eq!(b"world", &message1.content[..]);

    tokio::spawn(async move {
        let mut client = client::connect(addr).await.unwrap();
        client.publish("world", "howdy?".into()).await.unwrap()
    });
    let message2 = subscriber.next_message().await.unwrap().unwrap();
    assert_eq!("world", &message2.channel);
    assert_eq!(b"howdy?", &message2.content[..])
}

/// test that a client accurately removes its own subscribed channel list
/// when unsubscribing to all subscribed channels by submitting an empty vec
#[tokio::test]
async fn unsubscribes_from_all_channels() {
    let (addr, _) = start_server().await;

    let client = client::connect(addr.clone()).await.unwrap();
    let mut subscriber = client
        .subscribe(vec!["hello".into(), "world".into()])
        .await
        .unwrap();

    subscriber.unsubscribe(&[]).await.unwrap();
    assert_eq!(subscriber.get_subscribed().len(), 0);
}

#[tokio::test]
async fn unsubscribes_from_some_channels() {
    let (addr, _) = start_server().await;

    let client = client::connect(addr.clone()).await.unwrap();
    let mut subscriber = client
        .subscribe(vec!["hello".into(), "world".into(), "foo".into()])
        .await
        .unwrap();

    subscriber
        .unsubscribe(&["hello".into(), "world".into()])
        .await
        .unwrap();
    assert_eq!(subscriber.get_subscribed().len(), 1);
}

async fn start_server() -> (SocketAddr, JoinHandle<mini_redis::Result<()>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let handle = tokio::spawn(async move { server::run(listener, tokio::signal::ctrl_c()).await });
    (addr, handle)
}
