use std::{sync::Arc, time::SystemTime};

use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use reqwest::StatusCode;
use tracing::debug;

use crate::{
    get_session_id,
    models::{AppState, AuthUser, Session, User},
    tables::SESSIONS,
};

pub async fn auth(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Response {
    // Get session cookie
    let session_id = match get_session_id(&headers) {
        Some(id) => id,
        None => {
            debug!("could not get session id");
            return Redirect::to("/login").into_response();
        }
    };

    // Get session from database
    let session: Option<Session> = match state.db.select((SESSIONS, &session_id)).await {
        Ok(session) => session,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let session = match session {
        Some(session) => session,
        None => {
            debug!("could not get session");
            return Redirect::to("/login").into_response();
        }
    };

    // Check if session is expired
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if now > session.expires_at {
        debug!("session expired");
        return Redirect::to("/login").into_response();
    }

    // Get user from database
    let user: Option<User> = match state
        .db
        .select((&session.user_id.tb, &session.user_id.id.to_raw()))
        .await
    {
        Ok(user) => user,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let user = match user {
        Some(user) => user,
        None => {
            debug!("user not found");
            return Redirect::to("/login").into_response();
        }
    };

    let mut request = req;
    request.extensions_mut().insert(AuthUser(user));

    next.run(request).await
}
