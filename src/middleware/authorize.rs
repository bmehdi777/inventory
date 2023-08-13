use crate::{authentication::password::AuthenticationError, startup::AppStateRC, utils::AppError};
use axum::{extract::State, http::Request, middleware::Next, response::Response};
use axum_extra::extract::CookieJar;

pub async fn block_without_valid_cookie<B>(
    State(app_state): State<AppStateRC>,
    jar: CookieJar,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let cookie_uid = match jar.get("uid") {
        Some(cookie) => cookie.value(),
        None => {
            return Err(AppError::AuthenticationError(
                AuthenticationError::InvalidCookie,
            ))
        }
    };

    tracing::debug!(
        "has user id {:?}",
        app_state
            .session_store
            .has_user_id(cookie_uid.to_string())
            .await
    );
    if !app_state
        .session_store
        .has_user_id(cookie_uid.to_string())
        .await?
    {
        return Err(AppError::AuthenticationError(
            AuthenticationError::InvalidCookie,
        ));
    }
    Ok(next.run(request).await)
}
