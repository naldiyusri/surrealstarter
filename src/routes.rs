use std::sync::Arc;

use axum::{
    Json, Router,
    http::{HeaderValue, Method},
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::get,
};
use reqwest::{
    Client,
    header::{AUTHORIZATION, CONTENT_TYPE, SET_COOKIE},
};
use surrealdb::{Surreal, engine::any::Any};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::cors::CorsLayer;

use crate::{
    handlers, middleware,
    models::{ApiResponse, AppState},
};

pub async fn create_router(db: Surreal<Any>) -> Router {
    let state = Arc::new(AppState {
        db,
        client: Client::new(),
    });

    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(3)
            .burst_size(5)
            .finish()
            .unwrap(),
    );

    let governor_limiter = governor_conf.limiter().clone();
    let interval = std::time::Duration::from_secs(60);

    std::thread::spawn(move || {
        loop {
            std::thread::sleep(interval);
            // tracing::info!("rate limiting storage size: {}", governor_limiter.len());
            governor_limiter.retain_recent();
        }
    });

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3630".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([
            CONTENT_TYPE.to_string().parse().unwrap(),
            AUTHORIZATION.to_string().parse().unwrap(),
        ])
        .expose_headers([SET_COOKIE]);

    Router::new()
        .route("/login", get(handlers::discord::login))
        .route("/logout", get(handlers::discord::logout))
        .nest(
            "/api",
            Router::new()
                .route("/ping", get(ping))
                .route("/oauth/callback", get(handlers::discord::callback))
                .nest(
                    "/users",
                    Router::new().route(
                        "/@me",
                        get(handlers::discord::user)
                            .route_layer(from_fn_with_state(state.clone(), middleware::auth)),
                    ),
                ),
        )
        .layer(cors)
        .layer(GovernorLayer {
            config: governor_conf,
        })
        .with_state(state)
}

#[axum::debug_handler]
async fn ping() -> impl IntoResponse {
    Json(ApiResponse::<()> {
        data: (),
        message: "pong!".to_string(),
        errors: Vec::new(),
    })
}
