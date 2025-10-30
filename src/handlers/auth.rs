use axum::response::IntoResponse;
use crate::config::database::MongoDB;
use crate::db::AppState;
use crate::models::user::{AuthResponse, LoginRequest, RegisterRequest};
use crate::services::auth::AuthService;
use crate::utils::error::Result;
use crate::utils::response::ApiResponse;
use axum::{extract::State, http::StatusCode, Json};
use std::env;
use std::sync::Arc;

// POST /auth/register
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse> {
    let collection_name = MongoDB::get_collection_name("MONGO_USERS_COLLECTION");
    let collection = state.collection(&collection_name);

    let user = AuthService::register(&collection, req).await?;
    
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = AuthService::generate_jwt(&user, &jwt_secret)?;
    
    let user_response = AuthService::user_to_response(&user);

    let response = ApiResponse::success(AuthResponse {
        token,
        user: user_response,
    });

    Ok((StatusCode::CREATED, response))
}

// POST /auth/login
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    let collection_name = MongoDB::get_collection_name("MONGO_USERS_COLLECTION");
    let collection = state.collection(&collection_name);

    let user = AuthService::login(&collection, req).await?;
    
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = AuthService::generate_jwt(&user, &jwt_secret)?;
    
    let user_response = AuthService::user_to_response(&user);

    let response = ApiResponse::success(AuthResponse {
        token,
        user: user_response,
    });

    Ok(response)
}
