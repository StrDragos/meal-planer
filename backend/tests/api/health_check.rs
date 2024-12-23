use std::time::Duration;
use crate::common::*;
use crate::db::{migrate, with_db_container};

#[tokio::test]
async fn successful_health_check() {
    let _ = enable_tracing();

    with_db_container(|container, db_config| async move {

        let app_address = spawn_app(db_config).await;

        let client = reqwest::Client::new();
        let response = client.get(format!("{}/api/health_check", app_address))
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .expect("Failed to execute heath-check");

        assert!(response.status().is_success());
    }).await


}