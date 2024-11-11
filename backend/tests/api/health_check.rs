use std::time::Duration;
use backend::config::{AppConfig, FirebaseSettings};


#[tokio::test]
async fn successful_health_check() {
    spawn_app();

    let client = reqwest::Client::new();
    let response = client.get("http://127.0.0.1:8001/api/health_check")
        .timeout(Duration::from_secs(5))
        .send()
        .await
        .expect("Failed to execute heath-check");

    assert!(response.status().is_success());
}