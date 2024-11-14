use serde::Deserialize;
#[derive(Deserialize)]
pub struct AppConfig {
    pub port: Option<u16>,
    pub firebase_settings: FirebaseSettings
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct FirebaseSettings {
    pub project_id: String,
    pub api_key: String,
    pub admin_addr: String
}

impl AppConfig {
    pub fn get_config() -> Result<AppConfig, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::new("/config/app_properties.yaml", config::FileFormat::Yaml))
            .build()?;

        settings.try_deserialize::<AppConfig>()
    }
}