use axum::{extract::State, http::StatusCode, Json};
use mongodb::Database;

use crate::{
    routes::user::{User, USER_TABLENAME},
    utils::AppError,
};

pub async fn insert_user(
    State(db_client): State<Database>,
    Json(payload): Json<User>,
) -> Result<StatusCode, AppError> {
    db_client
        .collection::<User>(USER_TABLENAME)
        .insert_one(
            User {
                username: payload.username,
            },
            None,
        )
        .await
        .or_else(|_| Err(AppError::DuplicatedRessource))?;

    Ok(StatusCode::CREATED)
}
