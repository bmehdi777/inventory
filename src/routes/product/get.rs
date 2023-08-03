use crate::product::Product;
use crate::routes::PRODUCT_TABLENAME;
use crate::startup::DatabaseRC;
use crate::utils::AppError;
use axum::{extract::State, response::Json};
use futures::TryStreamExt;
use serde_json::{json, Value};

#[tracing::instrument(name = "GET products", skip(db_client))]
pub async fn get_products(State(db_client): State<DatabaseRC>) -> Result<Json<Value>, AppError> {
    tracing::info!("Querying list of product");
    let products: Vec<Product> = db_client
        .collection::<Product>(PRODUCT_TABLENAME)
        .find(None, None)
        .await?
        .try_collect()
        .await?;

    tracing::info!("Sending list of product");
    Ok(Json(json!(products)))
}

pub async fn get_product(State(db_client): State<DatabaseRC>) -> Result<Json<Value>, AppError> {
    unimplemented!()
}
