use futures::TryStreamExt;
use axum::{extract::State, Json};
use serde_json::{Value, json};

use crate::{routes::CATEGORY_TABLENAME, startup::AppStateRC, utils::AppError};

use super::Category;

#[axum::debug_handler]
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
