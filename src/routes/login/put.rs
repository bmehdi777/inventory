use axum::{extract::State, http::StatusCode, Json};
use mongodb::bson::doc;

use crate::{
    authentication::password::{create_hash_password, verify_password_hash},
    routes::{login::PasswordModify, user::User, USER_TABLENAME},
    startup::AppStateRC,
    utils::AppError,
};

#[tracing::instrument]
pub async fn modify_password(
    State(app_state): State<AppStateRC>,
    Json(payload): Json<PasswordModify>,
) -> Result<StatusCode, AppError> {
    match app_state
        .database
        .collection::<User>(USER_TABLENAME)
        .find_one(doc! { "uuid":&payload.uuid}, None)
        .await
        .unwrap_or(None)
    {
        Some(user) => {
            verify_password_hash(payload.current_password, user.password_hash)?;
            let updated = doc! { "$set": {"password_hash": create_hash_password(payload.new_password).await?}};
            app_state
                .database
                .collection::<User>(USER_TABLENAME)
                .update_one(doc! {"uuid": &payload.uuid}, updated, None).await?;
            return Ok(StatusCode::OK);
        }
        None => return Ok(StatusCode::UNAUTHORIZED),
    }
}
