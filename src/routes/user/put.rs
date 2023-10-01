use axum::{extract::State, http::StatusCode, Json};
use mongodb::bson::doc;

use crate::{
    routes::{
        user::{User, UsernameModify},
        USER_TABLENAME,
    },
    startup::AppStateRC,
    utils::AppError,
};


#[tracing::instrument]
pub async fn modify_username(
    State(app_state): State<AppStateRC>,
    Json(payload): Json<UsernameModify>,
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

