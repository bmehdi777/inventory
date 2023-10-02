use serde::{Deserialize, Serialize};

pub mod get;
pub mod put;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub picture: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsernameModify {
    pub new_username: String,
}

