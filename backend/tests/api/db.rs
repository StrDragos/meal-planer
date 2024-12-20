use std::future::Future;
use std::time::Duration;
use testcontainers::ContainerAsync;
use backend::config::DbConfig;
use sqlx::postgres::PgPoolOptions;
use testcontainers::runners::AsyncRunner;
use tracing::{debug, info};

pub async fn migrate(db_url: &str) {
    debug!("Migration for db_url:{}", db_url);
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect(db_url).await.unwrap();

    sqlx::migrate!("./migrations")
        .run(&pool).await.unwrap();
}

pub async fn with_db_container<T, Fut>(exec: T)
where
    T: FnOnce(ContainerAsync<testcontainers_modules::postgres::Postgres>, DbConfig) -> Fut,
    Fut: Future<Output = ()>,
{
    let container = testcontainers_modules::postgres::Postgres::default()
        .with_db_name("test_meal_plan")
        .with_password("test")
        .with_user("test")
        .start()
        .await
        .expect("Failed to start db");

    let port = container.get_host_port_ipv4(5432).await.unwrap();
    let config = DbConfig {
        user: "test".to_string(),
        password: "test".to_string(),
        port,
        db_name: "test_meal_plan".to_string(),
        host: "localhost".to_string(),
    };

    exec(container, config).await
}
