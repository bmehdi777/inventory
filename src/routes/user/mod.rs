use serde::{Deserialize, Serialize};

pub mod get;
pub mod put;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsernameModify {
    pub uuid: String,
    pub new_username: String,
}

