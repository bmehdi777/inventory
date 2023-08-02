use serde::{Deserialize, Serialize};

pub mod get;
pub mod post;
pub mod put;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub code_barre: Option<String>,
}
