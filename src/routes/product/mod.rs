use serde::{Deserialize, Serialize};

pub mod get;
pub mod post;
pub mod put;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub image: Option<String>,
    pub quantity: Option<u64>,
    pub code_barre: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductImage {
    pub base64_blob: String,
    pub quantity: u32,
}

impl From<OpenFoodFactProduct> for Product {
    fn from(product: OpenFoodFactProduct) -> Self {
        Product {
            name: product.product.name,
            image: Some(product.product.image),
            quantity: None,
            code_barre: Some(product.id),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenFoodFactProduct {
    #[serde(alias = "code")]
    pub id: String,

    pub product: OpenFoodFactProductDetail,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenFoodFactProductDetail {
    #[serde(alias = "product_name_fr")]
    pub name: String,

    #[serde(alias = "image_url")]
    pub image: String,
}
