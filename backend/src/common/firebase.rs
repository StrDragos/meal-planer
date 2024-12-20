use tracing::debug;
use crate::config::FirebaseSettings;

#[derive(Debug, Clone)]
pub struct Context {
    pub user_id: String,
}

#[derive(Clone, Debug)]
pub struct Firebase {
    pub settings: FirebaseSettings
}

impl Firebase {
    pub async fn get_user_data(&self, token: &str) -> Result<Context, reqwest::Error> {
        debug!("Received token {}", &token);
        Ok(Context { user_id: "test".to_string() })
    }
}


