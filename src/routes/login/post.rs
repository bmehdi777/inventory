use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use std::collections::BTreeMap;
use uuid::Uuid;

use crate::{
    authentication::password::{
        create_hash_password, validate_credentials, AuthenticationError, Credentials,
    },
    routes::{user::User, USER_TABLENAME},
    startup::AppStateRC,
    utils::AppError,
};
use jwt::SignWithKey;

use super::{JWTToken, RegisterPayload};

#[axum::debug_handler]
#[tracing::instrument]
pub async fn register(
    State(app_state): State<AppStateRC>,
    Json(payload): Json<RegisterPayload>,
) -> Result<StatusCode, AppError> {
    let user_id = Uuid::new_v4().to_string();
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
    app_state
        .database
        .collection::<User>(USER_TABLENAME)
        .insert_one(
            User {
                uuid: user_id.clone(),
                username: payload.username,
                email: payload.email,
                password_hash: create_hash_password(payload.password).await?,
                picture: None,
            },
            None,
        )
        .await
        .or_else(|_| Err(AppError::DuplicatedRessource))?;

    tracing::info!("Registering user : {} ", user_id.clone());
    // force user to login after registering (maybe the server will send mail to verify email ?)
    Ok(StatusCode::OK)
}

#[tracing::instrument]
pub async fn login(
    headers: HeaderMap,
    State(app_state): State<AppStateRC>,
) -> Result<Json<JWTToken>, AppError> {
    let auth_header = if let Some(header) = headers.get("Authorization") {
        header
    } else {
        return Err(AppError::AuthenticationError(
            AuthenticationError::InvalidCredentials,
        ));
    };
    let b64_encoded_auth = auth_header
        .to_str()
        .expect("The `Authorization` field was not a valid UTF8 string.")
        .strip_prefix("Basic ")
        .expect("Error while removing `basic`.");
    let decoded_auth =
        String::from_utf8(general_purpose::STANDARD.decode(b64_encoded_auth)?).unwrap();
    let mut split_auth = decoded_auth.splitn(2, ':');

    let creds: Credentials = Credentials {
        email: split_auth
            .next()
            .ok_or_else(|| anyhow::anyhow!("A username must be provided in 'Basic' auth"))?
            .to_string(),
        password: split_auth
            .next()
            .ok_or_else(|| anyhow::anyhow!("A password must be provided in 'Basic' auth"))?
            .to_string(),
    };
    tracing::info!("Validating credentials");
    let user_id = validate_credentials(creds, &app_state.database).await?;

    tracing::info!("user_id: {}", &user_id.to_string());

    match create_jwt(user_id.to_string(), &app_state.jwt_secret) {
        Ok(j) => return Ok(Json(j)),
        Err(e) => return Err(e),
    }
}

fn create_jwt(uid: String, key: &hmac::Hmac<sha2::Sha256>) -> Result<JWTToken, AppError> {
    let jwt_data: BTreeMap<String, String> = BTreeMap::from([
        (String::from("uid"), uid.to_string()),
        (
            String::from("validity"),
            (chrono::Utc::now() + chrono::Duration::days(31))
                .timestamp()
                .to_string(),
        ),
        (
            String::from("seed"),
            rand::thread_rng().gen_range(10000..99999).to_string(),
        ),
    ]);
    Ok(JWTToken {
        token: jwt_data.sign_with_key(key)?,
    })
}
