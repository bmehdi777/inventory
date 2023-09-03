use crate::{
    routes::{
        product::OpenFoodFactProduct,
        search::{SearchBarcodeRequest, SearchImageRequest},
        API_BARCODE,
    },
    utils::AppError,
};
use axum::Json;
use base64::{engine::general_purpose, Engine as _};
use rxing::helpers::detect_in_luma_with_hints;
use std::collections::HashMap;

#[tracing::instrument]
pub async fn search_product_by_image(
    Json(payload): Json<SearchImageRequest>,
) -> Result<Json<OpenFoodFactProduct>, AppError> {
    let blob = general_purpose::STANDARD.decode(payload.blob)?;
    let img = image::load_from_memory(&blob).expect("Error while loading image from blob.");

    let barcode = detect_in_luma_with_hints(
        img.to_luma8().to_vec(),
        img.width(),
        img.height(),
        None,
        &mut HashMap::new(),
    )?
    .getText()
    .to_string();
    tracing::info!("Barcode found : {}", barcode);

    let product_info = search_barcode(barcode).await?;
    tracing::info!("Product Info : {:?}", product_info);

    Ok(Json(product_info))
}

#[tracing::instrument]
pub async fn search_product_by_barcode(
    Json(payload): Json<SearchBarcodeRequest>,
) -> Result<Json<OpenFoodFactProduct>, AppError> {
    let product_info = search_barcode(payload.barcode).await?;
    Ok(Json(product_info))
}

async fn search_barcode(barcode: String) -> Result<OpenFoodFactProduct, AppError> {
    let url = format!("{}{}.json", API_BARCODE, barcode);
    tracing::info!("Contacting : {}", url);

    let product_info = reqwest::get(url)
        .await?
        .json::<OpenFoodFactProduct>()
        .await?;
    Ok(product_info)
}
