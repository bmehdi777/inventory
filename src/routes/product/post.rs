use crate::{
    routes::{product::Product, PRODUCT_TABLENAME},
    startup::AppStateRC,
    utils::AppError,
};
use axum::{extract::State, http::StatusCode, Json};

#[tracing::instrument]
pub async fn register_product(
    State(app_state): State<AppStateRC>,
    Json(payload): Json<Product>,
) -> Result<StatusCode, AppError> {
    app_state
        .database
        .collection::<Product>(PRODUCT_TABLENAME)
        .insert_one(payload, None)
        .await
        .or_else(|_| Err(AppError::DuplicatedRessource))?;
    Ok(StatusCode::OK)
}
