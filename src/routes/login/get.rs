use axum::extract::State;
use axum_extra::extract::CookieJar;
use http::StatusCode;

use crate::{startup::AppStateRC, utils::AppError};

#[tracing::instrument]
pub async fn disconnect(
    State(app_state): State<AppStateRC>,
    jar: CookieJar,
) -> Result<StatusCode, AppError> {
    let cookie_uid = match jar.get("uid") {
        Some(cookie) => cookie.value(),
        None => return Ok(StatusCode::OK),
    };
    tracing::info!("Found cookie uid: {}", &cookie_uid);
    app_state
        .session_store
        .log_out(cookie_uid.to_string())
        .await?;
    Ok(StatusCode::OK)
}
