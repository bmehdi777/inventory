use axum::{extract::State, http::StatusCode, Json};

use crate::{routes::CATEGORY_TABLENAME, startup::AppStateRC, utils::AppError};

use super::Category;

#[tracing::instrument]
pub async fn create(
    State(app_state): State<AppStateRC>,
    Json(payload): Json<Category>,
) -> Result<StatusCode, AppError> {
    app_state
        .database
        .collection::<Category>(CATEGORY_TABLENAME)
        .insert_one(payload, None)
        .await?;
    Ok(StatusCode::OK)
}
