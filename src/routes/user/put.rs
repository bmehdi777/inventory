use std::collections::BTreeMap;

use axum::{extract::State, http::StatusCode, Json, Extension};
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
    Extension(jwt_tok): Extension<BTreeMap<String,String>>,
    Json(payload): Json<UsernameModify>,
) -> Result<StatusCode, AppError> {
    let uid = if let Some(uid) = jwt_tok.get("uid") {
        uid
    } else {
        panic!();
    };
    let filter = doc! {"uuid" : uid};
    let updated = doc! {"$set" : {"username" : payload.new_username}};

    app_state
        .database
        .collection::<User>(USER_TABLENAME)
        .update_one(filter, updated, None)
        .await?;

    Ok(StatusCode::OK)
}
