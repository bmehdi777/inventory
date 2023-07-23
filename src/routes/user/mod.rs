use serde::{Deserialize, Serialize};

pub mod get;
pub mod post;
pub mod put;

const USER_TABLENAME: &'static str = "users";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
}
