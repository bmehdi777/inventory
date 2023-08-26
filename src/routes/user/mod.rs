use serde::{Deserialize, Serialize};

pub mod get;
pub mod put;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserModify {
    pub uuid: String,
    pub new_username: String,
}
