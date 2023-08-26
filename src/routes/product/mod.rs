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
pub struct ImageData {
    pub blob: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductRequest {
    pub image_data: Option<ImageData>,
    pub product_data: Option<Product>,
}

