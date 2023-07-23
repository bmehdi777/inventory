use serde::{Deserialize, Serialize};

pub mod get;
//mod post;
//mod put;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
}
