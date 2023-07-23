use futures::TryStreamExt;
use axum::{extract::State, Json};
use mongodb::Database;
use serde_json::{Value, json};

use crate::{utils::AppError, routes::user::{User, USER_TABLENAME}};

pub async fn get_users(State(db_client): State<Database>) -> Result<Json<Value>, AppError> {
    let users: Vec<User> = db_client.collection::<User>(USER_TABLENAME).find(None, None).await?.try_collect().await?;

    log::debug!("{:?}", users);

    Ok(Json(json!(users)))
}
