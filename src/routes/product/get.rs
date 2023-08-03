use crate::product::Product;
use crate::routes::PRODUCT_TABLENAME;
use crate::startup::DatabaseRC;
use crate::utils::AppError;
use axum::{extract::State, response::Json};
use futures::TryStreamExt;
use serde_json::{json, Value};

pub async fn get_products(State(db_client): State<DatabaseRC>) -> Result<Json<Value>, AppError> {
    let products: Vec<Product> = db_client
        .collection::<Product>(PRODUCT_TABLENAME)
        .find(None, None)
        .await?
        .try_collect()
        .await?;

    tracing::debug!("{:?}", products);

    Ok(Json(json!(products)))
}

pub async fn get_product(State(db_client): State<DatabaseRC>) -> Result<Json<Value>, AppError> {
    unimplemented!()
}
