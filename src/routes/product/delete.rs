use crate::product::{Product, ProductQuery};
use crate::routes::PRODUCT_TABLENAME;
use crate::startup::AppStateRC;
use axum::{
    extract::State,
    response::Json,
    http::StatusCode
};
use mongodb::bson::doc;

#[tracing::instrument(name = "GET products", skip(app_state))]
pub async fn delete(State(app_state): State<AppStateRC>, Json(payload): Json<ProductQuery>) -> StatusCode {
    match app_state.database.collection::<Product>(PRODUCT_TABLENAME).delete_one(doc!{"name": payload.name}, None).await {
        Ok(_) => return StatusCode::OK,
        Err(_) => return StatusCode::NO_CONTENT,
    }

}
