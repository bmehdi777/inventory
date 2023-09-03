use serde::{Serialize, Deserialize};

pub mod post;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchBarcodeRequest {
    pub barcode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchImageRequest {
    pub blob: String,
}
