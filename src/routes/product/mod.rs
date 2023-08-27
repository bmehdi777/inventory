use serde::{Deserialize, Serialize};

pub mod get;
pub mod post;
pub mod put;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub quantity: u64,
    pub code_barre: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductImage {
    pub base64_blob: String,
    pub quantity: u32
}
