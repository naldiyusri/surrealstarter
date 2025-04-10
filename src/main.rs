use axum::http::HeaderMap;
use reqwest::header;
use routes::create_router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod routes;

const PORT: u16 = 3631;

pub mod tables {
    pub const USERS: &str = "users";
    pub const SESSIONS: &str = "sessions";
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let address = std::env::var("SURREAL_ADDRESS").unwrap();
    let username = std::env::var("SURREAL_USERNAME").unwrap();
    let password = std::env::var("SURREAL_PASSWORD").unwrap();

    let db = db::connect(&address, &username, &password).await.unwrap();

    let app = create_router(db).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    tracing::debug!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

pub fn get_current_timestamp() -> u64 {
    let now = std::time::SystemTime::now();
    now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
}

/// Helper function to get session ID from cookies
pub fn get_session_id(headers: &HeaderMap) -> Option<String> {
    let cookie = headers.get(header::COOKIE)?;
    let cookie_str = cookie.to_str().ok()?;

    for cookie_pair in cookie_str.split("; ") {
        if let Some(session_id) = cookie_pair.strip_prefix("session_id=") {
            return Some(session_id.to_string());
        }
    }

    None
}
