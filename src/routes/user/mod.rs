use serde::{Deserialize, Serialize};

pub mod get;
pub mod post;
pub mod put;

const USER_TABLENAME: &'static str = "users";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPayload {
    pub username: String,
    pub password: String,
}
