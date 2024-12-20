use crate::common::{sign_in, with_firebase_auth_container};
use actix_web::dev::Server;
use actix_web::middleware::{from_fn, Logger};
use actix_web::web::Json;
use actix_web::{get, web, App, HttpMessage, HttpRequest, HttpServer, Responder};
use backend::common::firebase::{Firebase, Context};
use backend::config::FirebaseSettings;
use backend::middlwares::authorization::authorize;
use serde::{Deserialize, Serialize};
use std::net::TcpListener;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct TestResponse {
    user_id: String,
}

#[get("/test")]
async fn test_endpoint(req: HttpRequest) -> actix_web::Result<impl Responder> {
    if let Some(user_data) = req.extensions().get::<Context>() {
        Ok(Json(TestResponse { user_id: user_data.user_id.clone() }))
    } else {
        Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    }
}

pub async fn mock_app() -> Result<(Server, String), std::io::Error> {
    tracing_subscriber::fmt::init();
    let listener = TcpListener::bind("127.0.0.1:0").expect("Can't bind to port");
    let firebase = Firebase {
        settings: FirebaseSettings {
            project_id: "".to_string(),
            api_key: "".to_string(),
            admin_addr: "".to_string(),
        }
    };
    let server_port = listener.local_addr()?.port();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(from_fn(authorize(Arc::new(firebase.clone()))))
            .service(
                web::scope("/api")
                    .service(test_endpoint)
            )
    })
        .listen(listener)?
        .run();

    Ok((server, format!("http://localhost:{}", server_port)))
}

#[tokio::test]
async fn test_authorization() {
    with_firebase_auth_container(|admin_port| async move {
        println!("Port {}", admin_port);
        let (server, server_address) = mock_app().await.expect("Mock app to start");
        let _ = tokio::spawn(server);
        let token = sign_in("test@example.com".into(), "123ABC".into(), admin_port.to_string())
            .await.expect("unable to get user token");
        let http_client = reqwest::Client::new();
        let result = http_client
            .get(format!("{}/api/test", &server_address))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;
        let response = result.unwrap();
        let msg = response.json::<TestResponse>().await.unwrap();
        assert_eq!(msg.user_id, "test");
    }).await
}