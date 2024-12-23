use std::sync::Arc;
use crate::common::errors::app_error::AppError;
use crate::userdata::domain::diet::Diet;
use crate::userdata::domain::user_data::{SaveUserSettings, UserSettings};
use crate::userdata::repositories::diets_repo::DietRepository;

pub struct UserSettingsService {
    diet_repo: Arc<dyn DietRepository>
}

impl UserSettingsService {

    pub fn new(diet_repo: Arc<dyn DietRepository>) -> Self {
        Self {
            diet_repo
        }
    }
    pub async fn save(&self, _settings: SaveUserSettings) -> anyhow::Result<()> {
        Result::Ok(())
    }

    pub async fn  get(&self, user_id: String) -> anyhow::Result<UserSettings> {
        println!("user_id: {}", user_id);
        Ok(UserSettings {
            user_id: "test".to_string(),
            diet: None,
            no_of_persons: 2.into(),
        })
    }

    pub async fn get_diets(&self) -> Result<Vec<Diet>, AppError> {
        self.diet_repo.get_all().await.map_err(|err| AppError::from_db_error(err))
    }
}