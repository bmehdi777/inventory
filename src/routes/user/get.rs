use std::collections::BTreeMap;

use axum::{extract::State, Extension, Json};
use futures::TryStreamExt;
use mongodb::bson::doc;
use serde_json::{json, Value};

use crate::{
    routes::{user::User, USER_TABLENAME},
    startup::AppStateRC,
    utils::AppError,
};

#[tracing::instrument]
pub async fn get_users(State(app_state): State<AppStateRC>) -> Result<Json<Value>, AppError> {
    let users: Vec<User> = app_state
        .database
        .collection::<User>(USER_TABLENAME)
        .find(None, None)
        .await?
        .try_collect()
        .await?;

    tracing::debug!("{:?}", users);

    Ok(Json(json!(users)))
}

#[tracing::instrument]
pub async fn get_personal_info(
    State(app_state): State<AppStateRC>,
    Extension(jwt_tok): Extension<BTreeMap<String, String>>,
) -> Result<Json<Value>, AppError> {
    let uid = if let Some(uid) = jwt_tok.get("uid") {
        uid
    } else {
        return Err(AppError::UnexpectedError(anyhow::anyhow!("The uuid isn't found in databases while searching in get_personal_info but was found in the middleware.")));
    };
    match app_state
        .database
        .collection::<User>(USER_TABLENAME)
        .find_one(doc! {"uuid": uid}, None)
        .await.unwrap_or(None) {
            Some(user) => Ok(Json(json!(user))),
            None => Err(AppError::UnexpectedError(anyhow::anyhow!("The uuid isn't found in databases while searching in get_personal_info but was found in the middleware.")))
        }
}
