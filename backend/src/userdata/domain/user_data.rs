use crate::userdata::domain::diet::Diet;

#[derive(Debug, Clone)]
pub struct SaveUserSettings {
    pub diet: Option<Diet>,
    pub no_of_persons: Option<u8>
}

#[derive(Debug, Clone)]
pub struct UserSettings {
    pub user_id: String,
    pub diet: Option<Diet>,
    pub no_of_persons: Option<u8>,
}