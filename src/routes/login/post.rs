use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use uuid::Uuid;

use crate::{
    authentication::password::{create_hash_password, validate_credentials, Credentials},
    routes::{user::User, USER_TABLENAME},
    startup::AppStateRC,
    utils::AppError,
};

use super::UserPayload;

#[tracing::instrument]
pub async fn register(
    State(app_state): State<AppStateRC>,
    Json(payload): Json<UserPayload>,
) -> Result<(CookieJar, StatusCode), AppError> {
    let user_id = Uuid::new_v4().to_string();
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
    app_state
        .database
        .collection::<User>(USER_TABLENAME)
        .insert_one(
            User {
                uuid: user_id.clone(),
                username: payload.username,
                password_hash: create_hash_password(payload.password).await?,
            },
            None,
        )
        .await
        .or_else(|_| Err(AppError::DuplicatedRessource))?;
    tracing::info!("Successfully inserted user.");

    app_state
        .session_store
        .insert_user_id(user_id.clone())
        .await?;
    Ok((create_session_cookie(user_id), StatusCode::CREATED))
}

#[tracing::instrument]
pub async fn login(
    State(app_state): State<AppStateRC>,
    Json(payload): Json<UserPayload>,
) -> Result<(CookieJar, StatusCode), AppError> {
    let creds: Credentials = Credentials {
        username: payload.username,
        password: payload.password,
    };
    tracing::info!("Validating credentials");
    let user_id = validate_credentials(creds, &app_state.database).await?;

    tracing::info!("Inserting user_id");
    app_state
        .session_store
        .insert_user_id(user_id.to_string())
        .await?;
    Ok((create_session_cookie(user_id.to_string()), StatusCode::OK))
}

fn create_session_cookie(uid: String) -> CookieJar {
    CookieJar::new().add(
        Cookie::build("uid", format!("{}", uid))
            //.secure(true)
            .http_only(true)
            .finish(),
    )
}
