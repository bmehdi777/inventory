use crate::{
    routes::{
        product::{Product, ProductImage},
        PRODUCT_TABLENAME, API_BARCODE,
    },
    startup::AppStateRC,
    utils::AppError,
};
use axum::{extract::State, http::StatusCode, Json};
use base64::{engine::general_purpose, Engine as _};
use rxing::helpers::detect_in_luma_with_hints;
use std::collections::HashMap;

#[tracing::instrument]
pub async fn register_product_by_image(
    State(app_state): State<AppStateRC>,
    Json(payload): Json<ProductImage>,
) -> Result<StatusCode, AppError> {
    let blob = general_purpose::STANDARD.decode(payload.base64_blob)?;
    let img = image::load_from_memory(&blob).unwrap();
    let mut hints: rxing::DecodingHintDictionary = HashMap::new();

    let data = detect_in_luma_with_hints(
        img.to_luma8().to_vec(),
        img.width(),
        img.height(),
        None,
        &mut hints,
    );

    match data {
        Ok(o) => {
            tracing::info!("Codebar retrieved : {}", o);
            let url = format!("{}{}.json", API_BARCODE, o);
            let product_info: Product = app_state.reqwest_client.get(url).send().await?.json().await?;
            tracing::info!("Product Info : {:?}", product_info);

        },
        Err(e) => tracing::error!("Error encountered while detecting codebar in image : {}", e),
    }
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
