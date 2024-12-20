use anyhow::anyhow;
use backend::config::{AppConfig, DbConfig};
use futures_util::TryFutureExt;
use rs_firebase_admin_sdk::auth::User;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::io;
use std::io::ErrorKind;
use std::net::TcpListener;
use std::thread::sleep;
use std::time::Duration;
use testcontainers::core::{ContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};
use testcontainers_modules::postgres::Postgres;
use tracing::{debug, error};

pub fn default_db_config() -> DbConfig {
    DbConfig {
        db_name: "meal_plan".to_string(),
        host: "localhost".to_string(),
        password: "<PASSWORD>".to_string(),
        port: 5432,
        user: "meal_plan".to_string(),
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LoginReq {
    email: String,
    password: String,
    return_secure_token: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoginResp {
    #[allow(dead_code)]
    pub id_token: String,
}

pub async fn spawn_app(db_config: DbConfig) -> String {
    let app_config = AppConfig {
        port: None,
        firebase_settings: Default::default(),
        db_config,
    };

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = backend::run(listener, app_config)
        .await
        .expect("Failed to start server");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

//TODO: add a high order function to work with the container, make sure the container stays alive in the scope.
pub async fn firebase_auth_container(
) -> anyhow::Result<(ContainerAsync<GenericImage>, Option<u16>, Option<u16>)> {
    let container = GenericImage::new("kpetrikas/firebase-emulator", "latest")
        // .with_wait_for(WaitFor::Log(""))
        .with_exposed_port(ContainerPort::Tcp(4000))
        .with_exposed_port(ContainerPort::Tcp(9099))
        .with_env_var("PROJECT_ID", "meal-planer-dev")
        .with_env_var("EMULATORS", "auth")
        .start()
        .await
        .expect("Failed to start server");

    let ports = container
        .ports()
        .await
        .expect("Cound not get available ports");
    let ui_port = ports.map_to_host_port_ipv4(ContainerPort::Tcp(4000));
    let auth_port = ports.map_to_host_port_ipv4(ContainerPort::Tcp(9099));
    Ok((container, ui_port, auth_port))
}

type AuthPort = u16;
pub async fn with_firebase_auth_container<F, Fut>(f: F)
where
    F: FnOnce(AuthPort) -> Fut,
    Fut: Future<Output = ()>,
{

    let wait_strategy = WaitFor::message_on_stdout(
        "Issues? Report them at https://github.com/firebase/firebase-tools/issues and attach the *-debug.log files."
    );
    let container = GenericImage::new("kpetrikas/firebase-emulator", "latest")
        .with_wait_for(wait_strategy)
        .with_exposed_port(ContainerPort::Tcp(4000))
        .with_exposed_port(ContainerPort::Tcp(9099))
        .with_env_var("PROJECT_ID", "meal-planer-dev")
        .with_env_var("EMULATORS", "auth")
        .start()
        .await
        .expect("Failed to start server");

    let ports = container
        .ports()
        .await
        .expect("Couldn't get available ports");
    let auth_port = ports
        .map_to_host_port_ipv4(ContainerPort::Tcp(9099))
        .expect("Couldn't not get available ports");
    f(auth_port).await;
}

pub async fn sign_in(email: String, password: String, port: String) -> Result<String, io::Error> {
    let client = reqwest::Client::builder().build().unwrap();
    let firebase_login = format!(
        "http://localhost:{}/identitytoolkit.googleapis.com/v1/accounts:signUp?key=123",
        port
    );
    let mut attemts = 0;

    while attemts < 10 {
        let response = client
            .post(firebase_login.clone())
            .header("content-type", "application/json")
            .json(&LoginReq {
                email: email.clone(),
                password: password.clone(),
                return_secure_token: true,
            })
            .send()
            .await;
        match response {
            Ok(r) => {
                let login_resp: LoginResp = r.json().await.unwrap();
                return Ok(login_resp.id_token);
            }
            Err(err) => {
                debug!("Failed to get token try one more time");
                error!("{:#?}", err);
            }
        }

        attemts += 1;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    Err(ErrorKind::Other.into())
}
