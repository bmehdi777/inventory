use serde::{Deserialize, Serialize};

pub mod post;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPayload {
    pub username: String,
    pub password: String,
}
