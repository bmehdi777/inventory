use crate::{authentication::password::AuthenticationError, startup::AppStateRC, utils::AppError};
use axum::{extract::State, http::HeaderMap, http::Request, middleware::Next, response::Response};
use chrono::{TimeZone, Utc};
use jwt::VerifyWithKey;
use std::collections::BTreeMap;

pub async fn block_without_valid_jwt<B>(
    header: HeaderMap,
    State(app_state): State<AppStateRC>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let jwt_str: &str = match header.get("Authorization") {
        Some(j) => j
            .to_str()
            .expect("An error occured while parsing jwt to str"),
        None => return Err(AppError::AuthenticationError(AuthenticationError::NoJWT)),
    };
    let jwt_without_bearer = jwt_str
        .strip_prefix("Bearer ")
        .expect("The Authentication field doesn't contains Bearer.");

    let jwt_tok: BTreeMap<String, String> =
        jwt_without_bearer.verify_with_key(&app_state.jwt_secret)?;

    let validity = Utc.timestamp_opt(
        jwt_tok
            .get("validity")
            .expect("Validity field should have been in the jwt.")
            .parse::<i64>()
            .expect("Validity isn't an intm"),
        0,
    ).unwrap();

    if Utc::now().cmp(&validity).is_gt() {
        return Err(AppError::JWTExpired);
    }

    request.extensions_mut().insert(jwt_tok);

    Ok(next.run(request).await)
}
