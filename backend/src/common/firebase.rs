use serde::Deserialize;
use crate::config::FirebaseSettings;

#[derive(Debug, Clone)]
pub struct UserData {
    pub user_id: String,
}

#[derive(Clone, Debug)]
pub struct Firebase {
    pub settings: FirebaseSettings
}

impl Firebase {
    pub async fn get_user_data(&self, token: &str) -> Result<UserData, reqwest::Error> {
        println!("Received token {}", token);
        Ok(UserData { user_id: "test".to_string() })
    }
}


