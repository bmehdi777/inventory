use crate::product::{Product, ProductQuery};
use crate::routes::PRODUCT_TABLENAME;
use crate::startup::AppStateRC;
use crate::utils::AppError;
use axum::{
    extract::State,
    response::Json,
};
use futures::TryStreamExt;
use mongodb::bson::doc;
use serde_json::{json, Value};

#[tracing::instrument(name = "GET products", skip(app_state))]
pub async fn get_products(State(app_state): State<AppStateRC>) -> Result<Json<Value>, AppError> {
    tracing::info!("Querying list of product");
    let products: Vec<Product> = app_state
        .database
        .collection::<Product>(PRODUCT_TABLENAME)
        .find(None, None)
        .await?
        .try_collect()
        .await?;

    tracing::info!("Sending list of product");
    Ok(Json(json!(products)))
}

pub async fn get_product(
    State(app_state): State<AppStateRC>,
    Json(product_query): Json<ProductQuery>,
) -> Result<Json<Product>, AppError> {
    tracing::info!("Queyring a product");
    match app_state
        .database
        .collection::<Product>(PRODUCT_TABLENAME)
        .find_one(doc! {"name": product_query.name }, None)
        .await
        .unwrap_or(None)
    {
        Some(product) => Ok(Json(product)),
        None => Err(AppError::NoRessource),
    }
}
