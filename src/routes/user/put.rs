use axum::{extract::State, http::StatusCode, Json};
use mongodb::bson::doc;

use crate::{
    routes::{
        user::{User, UserModify},
        USER_TABLENAME,
    },
    startup::AppStateRC,
    utils::AppError,
};

#[tracing::instrument]
pub async fn modify_user(
    State(app_state): State<AppStateRC>,
    Json(payload): Json<UserModify>,
) -> Result<StatusCode, AppError> {
    let filter = doc! {"uuid" : payload.uuid};
    let updated = doc! {"$set" : {"username" : payload.new_username}};

    app_state
        .database
        .collection::<User>(USER_TABLENAME)
        .update_one(filter, updated, None)
        .await?;

    Ok(StatusCode::OK)
}
