use std::{collections::HashMap, env, net::SocketAddr, sync::Arc, time::SystemTime};

use axum::{
    Extension, Json,
    extract::{ConnectInfo, Query, State},
    http::HeaderMap,
    response::{IntoResponse, Redirect},
};
use reqwest::{StatusCode, header};
use serde_json::Value;
use surrealdb::sql::Id;
use surrealdb::sql::Thing;
use tracing::error;

use crate::{
    get_session_id,
    models::{ApiResponse, AppState, AuthUser, OAuthQuery, Session, TokenResponse, User},
    tables::{SESSIONS, USERS},
};

// Redirect to Discord OAuth
#[axum::debug_handler]
pub async fn login() -> impl IntoResponse {
    let client_id = env::var("DISCORD_CLIENT_ID").expect("Missing DISCORD_CLIENT_ID");
    let redirect_uri = env::var("DISCORD_REDIRECT_URI").expect("Missing REDIRECT_URI");

    let discord_url = format!(
        "https://discord.com/api/oauth2/authorize?client_id={}&response_type=code&redirect_uri={}&scope=identify+email",
        client_id, redirect_uri
    );

    Redirect::to(&discord_url)
}

// Handle the OAuth callback
#[axum::debug_handler]
pub async fn callback(
    Query(params): Query<OAuthQuery>,
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    // Check if we got an error or missing code
    if params.error.is_some() || params.code.is_none() {
        // return (StatusCode::BAD_REQUEST, "Authorization failed").into_response();
        return Redirect::to("/login").into_response();
    }

    let code = params.code.unwrap();

    // Exchange code for token
    let client_id = env::var("DISCORD_CLIENT_ID").expect("Missing DISCORD_CLIENT_ID");
    let client_secret = env::var("DISCORD_CLIENT_SECRET").expect("Missing DISCORD_CLIENT_SECRET");
    let redirect_uri = env::var("DISCORD_REDIRECT_URI").expect("Missing REDIRECT_URI");

    let mut params = HashMap::new();
    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert("grant_type", "authorization_code".to_string());
    params.insert("code", code);
    params.insert("redirect_uri", redirect_uri);

    let token_res = state
        .client
        .post("https://discord.com/api/oauth2/token")
        .form(&params)
        .send()
        .await;

    // Handle token response
    let token = match token_res {
        Ok(response) => match response.json::<TokenResponse>().await {
            Ok(token) => token,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::<()> {
                        data: (),
                        message: "Failed to parse token".to_string(),
                        errors: vec![e.to_string()],
                    }),
                )
                    .into_response();
            }
        },
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()> {
                    data: (),
                    message: "Failed to get token".to_string(),
                    errors: vec![e.to_string()],
                }),
            )
                .into_response();
        }
    };

    // Get user data with token
    let user_res = state
        .client
        .get("https://discord.com/api/users/@me")
        .header(
            header::AUTHORIZATION,
            format!("{} {}", token.token_type, token.access_token),
        )
        .send()
        .await;

    // Handle user data response
    let user = match user_res {
        Ok(response) => match response.json::<Value>().await {
            Ok(user) => user,
            Err(_) => {
                // return (
                //     StatusCode::INTERNAL_SERVER_ERROR,
                //     "Failed to parse user data",
                // )
                //     .into_response();

                return Redirect::to("/login").into_response();
            }
        },
        Err(_) => {
            return Redirect::to("/login").into_response();
            // return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get user data").into_response();
        }
    };

    // Store user in database
    match state
        .db
        .create::<Option<User>>((USERS, user["id"].as_str().unwrap()))
        .content(user.clone())
        .await
    {
        Ok(_) => (),
        Err(e) => error!("Database error: {:?}", e),
    }

    let session_id = Id::ulid().to_string();
    let expires_at = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 7 * 24 * 60 * 60; // One week expiry

    let session = Session {
        id: Thing::from((SESSIONS, session_id.as_str())),
        user_id: Thing::from((USERS, user["id"].as_str().unwrap())),
        ipv4: addr.ip().to_string(),
        expires_at,
    };

    match state
        .db
        .create::<Option<Session>>((SESSIONS, &session_id))
        .content(session.clone())
        .await
    {
        Ok(_) => (),
        Err(e) => {
            error!("Database error: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()> {
                    data: (),
                    message: "Couldn't create the session in the database".to_string(),
                    errors: vec![e.to_string()],
                }),
            )
                .into_response();
        }
    }

    let mut response = (Json(ApiResponse::<Session> {
        data: session,
        message: "Session created successfully".to_string(),
        errors: Vec::new(),
    }))
    .into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        format!(
            "session_id={}; Path=/; HttpOnly; Max-Age={}",
            session_id, expires_at
        )
        .parse()
        .unwrap(),
    );

    response
}

#[axum::debug_handler]
pub async fn logout(headers: HeaderMap, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let session_id = match get_session_id(&headers) {
        Some(id) => id,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    data: (),
                    message: "You have no active sessions".to_string(),
                    errors: Vec::new(),
                }),
            )
                .into_response();
        }
    };

    let _: Option<Session> = state
        .db
        .delete((SESSIONS, &session_id))
        .await
        .unwrap_or(None);

    let mut response = (Json(ApiResponse {
        data: (),
        message: "Session deleted successfully".to_string(),
        errors: Vec::new(),
    }))
    .into_response();

    response.headers_mut().insert(
        header::SET_COOKIE,
        "session_id=; Path=/; HttpOnly; Max-Age=0".parse().unwrap(),
    );

    response
}

/// Display user data
#[axum::debug_handler]
pub async fn user(
    State(_state): State<Arc<AppState>>,
    Extension(auth_user): Extension<AuthUser>,
) -> impl IntoResponse {
    let user = auth_user.0;
    Json(user).into_response()
}
