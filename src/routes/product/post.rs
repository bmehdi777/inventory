use crate::{
    routes::product::ProductRequest,
    startup::AppStateRC,
    utils::AppError,
};
use axum::{extract::State, http::StatusCode, Json};
use rxing::helpers::detect_in_luma;

#[tracing::instrument]
pub async fn register_product(
    State(app_state): State<AppStateRC>,
    Json(payload): Json<ProductRequest>,
) -> Result<StatusCode, AppError> {

    if let Some(data_img) = payload.image_data {
        let data = detect_in_luma(data_img.blob, data_img.width, data_img.height, None);
        return Ok(StatusCode::OK)
    }
    if let Some(product) = payload.product_data {

    }

    Ok(StatusCode::UNPROCESSABLE_ENTITY)
}

