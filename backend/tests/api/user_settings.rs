use crate::common::{default_db_config, enable_tracing, sign_in, spawn_app, with_firebase_auth_container};
use serde_json::json;
use tracing::field::debug;
use tracing::log::debug;
use crate::db::{migrate, with_db_container};
use backend::userdata::domain::diet::Diet;



#[tokio::test]
async fn get_diets() {
    let _ = enable_tracing();

    with_firebase_auth_container(|&admin_port| async move {
        with_db_container(|container, db_config| async move {
            let _ = migrate(&db_config.postgres_url()).await;
            let address = spawn_app(db_config.clone()).await;
            let token = sign_in(
                "test@example.com".into(),
                "123ABC".into(),
                admin_port.to_string(),
            )
                .await
                .expect("unable to get user token");

            let client = reqwest::Client::new();
            let response = client
                .get(format!("{}/api/v1/settings/diets", address))
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await
                .expect("Should be able to get diets");
            let body = response.text().await.expect("Should be able to get diets");
            let results =
                serde_json::from_str::<Vec<Diet>>(&body).expect("Should be able to get diets");
            assert_eq!(results.is_empty(), false)
        }).await;
    })
    .await
}

#[tokio::test]
async fn save_settings() {
    let _ = enable_tracing();

    with_firebase_auth_container(|&admin_port| async move {
        with_db_container(|_, db_settings| async move {
            debug!("db test settings: {:?}", db_settings);
            let _ = migrate(&db_settings.postgres_url()).await;
            let address = spawn_app(db_settings).await;
            let token = sign_in(
                "test@example.com".into(),
                "123ABC".into(),
                admin_port.to_string(),
            )
            .await
            .expect("unable to get user token");

            let client = reqwest::Client::new();
            let input = json!({
                "diet": "Ketogenic",
                "no_of_persons": 2
            });

            let response = client
                .get(format!("{}/api/v1/settings", address))
                .json(&input)
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await
                .expect("Should be able to save settings");

            assert_eq!(response.status(), reqwest::StatusCode::OK);
        })
        .await
    })
    .await
}
