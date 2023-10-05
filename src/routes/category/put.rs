use axum::{extract::State, http::StatusCode, Json};
use mongodb::bson::doc;

use crate::{routes::CATEGORY_TABLENAME, startup::AppStateRC, utils::AppError};

use super::{Category, ChangeCategory};

#[tracing::instrument]
pub async fn update(
    State(app_state): State<AppStateRC>,
    Json(payload): Json<ChangeCategory>,
) -> Result<StatusCode, AppError> {
    app_state
        .database
        .collection::<Category>(CATEGORY_TABLENAME)
        .update_one(
            doc! {"name": payload.current_name},
            doc! {"name": payload.new_name},
            None,
        )
        .await?;
    Ok(StatusCode::OK)
}
