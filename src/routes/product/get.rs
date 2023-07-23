use crate::product::Product;
use crate::utils::AppError;
use axum::{extract::State, http::StatusCode, response::Json};
use futures::TryStreamExt;
use mongodb::{bson::Document, Database};
use serde_json::{json, Value};

pub async fn get_products(State(db_client): State<Database>) -> Result<Json<Value>, AppError> {
    let cursor: Vec<Product> = db_client
        .collection::<Product>("product")
        .find(None, None)
        .await?
        .try_collect()
        .await?;

    log::info!("{:?}", cursor);

    Ok(Json(json!(cursor)))
}

pub async fn get_product(State(db_client): State<Database>) -> Result<Json<Value>, AppError> {
    unimplemented!()
}
