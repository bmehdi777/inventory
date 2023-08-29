use crate::{
    routes::{
        product::{Product, ProductImage},
        API_BARCODE, PRODUCT_TABLENAME,
    },
    startup::AppStateRC,
    utils::AppError,
};
use axum::{extract::State, http::StatusCode, Json};
use base64::{engine::general_purpose, Engine as _};
use rxing::helpers::detect_in_luma_with_hints;
use std::collections::HashMap;

#[axum::debug_handler]
pub async fn register_product_by_image(
    State(app_state): State<AppStateRC>,
    Json(payload): Json<ProductImage>,
) -> Result<StatusCode, AppError> {
    let blob = general_purpose::STANDARD.decode(payload.base64_blob)?;
    let img = image::load_from_memory(&blob).expect("Error while loading image from blob.");

    let data = detect_in_luma_with_hints(
        img.to_luma8().to_vec(),
        img.width(),
        img.height(),
        None,
        &mut HashMap::new(),
    )?
    .getText()
    .to_string();
    tracing::info!("Barcode found : {}", data);

    let url = format!("{}{}.json", API_BARCODE, data);
    tracing::info!("Contacting : {}", url);
    let product_info = reqwest::get(url).await?.json::<Product>().await?;
    tracing::info!("Product Info : {:?}", product_info);
    return Ok(StatusCode::OK);
}

#[tracing::instrument]
pub async fn register_product_by_information(
    State(app_state): State<AppStateRC>,
    Json(payload): Json<Product>,
) -> Result<StatusCode, AppError> {
    app_state
        .database
        .collection::<Product>(PRODUCT_TABLENAME)
        .insert_one(payload, None)
        .await?;
    Ok(StatusCode::OK)
}
