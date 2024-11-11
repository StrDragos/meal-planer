use backend::config::{AppConfig, FirebaseSettings};

pub fn spawn_app() {
    let test_settings = AppConfig{
        port: Some(8001),
        firebase_settings: FirebaseSettings{
            project_id: "Test".to_string(),
            api_key: "test".to_string()
        }
    };

    let server = backend::run(&test_settings).expect("Failed to start server");
    let _ = tokio::spawn(server);
}
