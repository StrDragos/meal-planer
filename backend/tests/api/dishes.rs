use crate::helpers::spanw_app;

#[tokio::test]
async fn get_list_of_dishes() {
    spawn_app();

    let client = reqwest::Client::new();
    let response = client.get("http://127.0.0.1:8080/api/dishes")
        .send()
        .await
        .expect("Failed to execute heath-check");

    assert!(response.status().is_success());
}