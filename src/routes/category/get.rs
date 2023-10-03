use axum::{extract::State, http::StatusCode, Json};

use crate::{routes::CATEGORY_TABLENAME, startup::AppStateRC, utils::AppError};

use super::Category;

#[tracing::instrument]
pub async fn get_categories(State(app_state): State<AppStateRC>) -> Result<Json<Value>, AppError> {
    let categories: Vec<Category> = app_state
        .database
        .collection::<Category>(CATEGORY_TABLENAME)
        .find(None, None)
        .await?
        .try_collect()
        .await?;

    Ok(Json(json!(categories)))
}
