use reqwest::Client;
use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::any::Any, sql::Thing};

#[derive(Clone)]
pub struct AppState {
    pub db: Surreal<Any>,
    pub client: Client,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub message: String,
    pub errors: Vec<String>,
}

// Discord

#[derive(Debug, Deserialize)]
pub struct OAuthQuery {
    pub code: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Thing,
    pub global_name: Option<String>,
    pub username: String,
    pub avatar: Option<String>,
    pub discriminator: String,
    pub email: String,
    pub locale: String,
    pub mfa_enabled: bool,
    pub banner_color: Option<String>,
    pub accent_color: Option<u64>,
    pub verified: bool,
    pub flags: u64,
    pub premium_type: u64,
    pub public_flags: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Thing,
    pub user_id: Thing,
    pub ipv4: String,
    pub expires_at: u64,
}

#[derive(Clone)]
pub struct AuthUser(pub User);
