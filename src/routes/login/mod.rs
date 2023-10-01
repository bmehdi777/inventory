use serde::{Deserialize, Serialize};

pub mod post;
pub mod put;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPayload {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordModify {
    pub uuid: String,
    pub current_password: String,
    pub new_password: String,
}
