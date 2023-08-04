use crate::startup::AppStateRC;
use axum::{extract::State, http::StatusCode, middleware::Next, response::Response};
use axum_extra::extract::CookieJar;

pub async fn block_or_continue<B>(
    State(app_state): State<AppStateRC>,
    jar: CookieJar,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let cookie = match jar.get("uid") {
        Some(cookie) => cookie,
        None => return Err(StatusCode::UNAUTHORIZED)
    };

    tracing::info!("has user id {:?}",app_state.session_store.has_user_id(cookie.to_string()).await);
    Err(StatusCode::UNAUTHORIZED)
}
