use std::time::Duration;
use crate::common::*;

#[tokio::test]
async fn successful_health_check() {
    let app_address = spawn_app();

    let client = reqwest::Client::new();
    let response = client.get(format!("{}/api/health_check", app_address))
        .timeout(Duration::from_secs(5))
        .send()
        .await
        .expect("Failed to execute heath-check");

    assert!(response.status().is_success());
}