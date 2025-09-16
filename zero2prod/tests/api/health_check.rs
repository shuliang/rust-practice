use crate::helpers::{spawn_app, TestApp};

#[tokio::test]
async fn health_check_works() {
    let app: TestApp = spawn_app().await;
    let address = app.address;
    let client = reqwest::Client::new();

    println!("bind to address: {:?}", address);
    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
