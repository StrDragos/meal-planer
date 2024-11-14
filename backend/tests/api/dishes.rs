use crate::common::*;
use reqwest::StatusCode;
use std::string::ToString;

#[tokio::test]
async fn get_list_of_dishes() {
    let client = reqwest::Client::new();
    //TODO pass closure to firebase auth to execute test in the context of the container it will not require to return a container
    let (container, _, admin_port) = firebase_auth_container().await.expect("successful container to firebase api");

    let token = sign_in("test@example.com".into(), "123ABC".into(), admin_port.unwrap().to_string())
        .await.expect("unable to get user token");

    let app_address = spawn_app();

    let response = client
        .get(format!("{}/api/v1/dishes", app_address))
        .header("Authorization", token)
        .send()
        .await
        .expect("Failed to execute heath-check");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn fail_unauthorized_request () {
    let app_address = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/api/v1/dishes", app_address))
        .send()
        .await
        .expect("Failed to execute heath-check");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}