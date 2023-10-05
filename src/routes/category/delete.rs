use super::Category;
use crate::{routes::CATEGORY_TABLENAME, startup::AppStateRC};
use axum::{extract::State, http::StatusCode, Json};
use mongodb::bson::doc;

#[axum::debug_handler]
#[tracing::instrument]
pub async fn delete(
    State(app_state): State<AppStateRC>,
    Json(payload): Json<Category>,
) -> StatusCode {
    match app_state
        .database
        .collection::<Category>(CATEGORY_TABLENAME)
        .delete_one(doc! { "name": payload.name }, None)
        .await
    {
        Ok(_) => return StatusCode::OK,
        Err(_) => return StatusCode::NO_CONTENT,
    }
}
