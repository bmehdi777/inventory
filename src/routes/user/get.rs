use axum::{extract::State, Json};
use futures::TryStreamExt;
use serde_json::{json, Value};

use crate::{
    routes::{user::User, USER_TABLENAME},
    startup::AppStateRC,
    utils::AppError,
};

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
