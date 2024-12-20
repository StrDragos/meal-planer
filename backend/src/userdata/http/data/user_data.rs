
use crate::userdata::domain::user_data::SaveUserSettings;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use crate::userdata::domain::diet::Diet;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    diet: Option<Diet>,
    no_of_persons: Option<u8>,
}

impl From<UserData> for SaveUserSettings {
    fn from(value: UserData) -> Self {
        SaveUserSettings {
            diet: value.diet,
            no_of_persons: value.no_of_persons,
        }
    }
}