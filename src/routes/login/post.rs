use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use uuid::Uuid;

use crate::{
    authentication::password::{create_hash_password, validate_credentials, Credentials},
    routes::{user::User, USER_TABLENAME},
    startup::DatabaseRC,
    utils::AppError,
};

use super::UserPayload;

pub async fn register(
    State(db_client): State<DatabaseRC>,
    Json(payload): Json<UserPayload>,
) -> Result<(CookieJar, StatusCode), AppError> {
    let user_id = Uuid::new_v4().to_string();
    db_client
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

    Ok((
        create_cookie_session(user_id.to_string()),
        StatusCode::CREATED,
    ))
}

pub async fn login(
    State(db_client): State<DatabaseRC>,
    Json(payload): Json<UserPayload>,
) -> Result<(CookieJar, StatusCode), AppError> {
    let creds: Credentials = Credentials {
        username: payload.username,
        password: payload.password,
    };
    let user_id = validate_credentials(creds, db_client).await?;

    Ok((create_cookie_session(user_id.to_string()), StatusCode::OK))
}

fn create_cookie_session(uid: String) -> CookieJar {
    CookieJar::new().add(
        Cookie::build("uid", format!("{}", uid))
            .secure(true)
            .http_only(true)
            .finish(),
    )
}
