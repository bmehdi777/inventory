use axum::{extract::State, http::StatusCode, Json};
use mongodb::Database;

use crate::{
    authentication::password::{Credentials,create_hash_password, validate_credentials},
    routes::user::{User, USER_TABLENAME},
    utils::AppError,
};

use super::UserPayload;

pub async fn register(
    State(db_client): State<Database>,
    Json(payload): Json<UserPayload>,
) -> Result<StatusCode, AppError> {
    db_client
        .collection::<User>(USER_TABLENAME)
        .insert_one(
            User {
                uuid: uuid::Uuid::new_v4().to_string(),
                username: payload.username,
                password_hash: create_hash_password(payload.password).await?,
            },
            None,
        )
        .await
        .or_else(|_| Err(AppError::DuplicatedRessource))?;

    Ok(StatusCode::CREATED)
}

pub async fn login(State(db_client): State<Database>, Json(payload): Json<UserPayload>) -> Result<StatusCode, AppError> {
    let creds: Credentials = Credentials {username: payload.username, password: payload.password};
    validate_credentials(creds, db_client).await?;
    Ok(StatusCode::OK)
}
