use serde::Deserialize;
#[derive(Deserialize)]
pub struct AppConfig {
    pub port: Option<u16>,
    pub firebase_settings: FirebaseSettings,
    pub db_config: DbConfig,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct FirebaseSettings {
    pub project_id: String,
    pub api_key: String,
    pub admin_addr: String
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db_name: String
}

impl DbConfig {
    pub fn postgres_url(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.db_name)
    }
}

impl AppConfig {
    pub fn get_config() -> Result<AppConfig, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::new("/config/app_properties.yaml", config::FileFormat::Yaml))
            .build()?;

        settings.try_deserialize::<AppConfig>()
    }
}