use serde::{Deserialize, Serialize};

pub mod post;
pub mod get;
pub mod delete;
pub mod put;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangeCategory {
    pub current_name: String,
    pub new_name: String,
}
