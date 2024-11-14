use backend::config::AppConfig;
use backend::run;
use std::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();
    let config = AppConfig::get_config().expect("Error getting config");
    let port = config.port.unwrap_or(0);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", &port)).expect("Can't bind to port");
    info!("Running app on port {}", &port);
    run(listener, config)?.await
}
