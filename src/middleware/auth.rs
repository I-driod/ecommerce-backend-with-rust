use crate::models::user::Claims;
use crate::services::auth::AuthService;
use crate::utils::error::AppError;
use axum::{
    extract::{Request, State, FromRequestParts},
    middleware::Next,
    response::Response,
    http::request::Parts,
};
use std::sync::Arc;

pub async fn auth_middleware(
    State(jwt_secret): State<Arc<String>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extract Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::AuthError("Missing authorization header".to_string()))?;

    // Check for Bearer prefix
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::AuthError("Invalid authorization format".to_string()))?;

    // Verify JWT
    let claims = AuthService::verify_jwt(token, &jwt_secret)?;

    // Insert claims into request extensions
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

// Extractor for getting authenticated user from request
#[derive(Debug, Clone)]
pub struct AuthUser {
    #[allow(dead_code)]
    pub claims: Claims,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let claims = parts
            .extensions
            .get::<Claims>()
            .cloned()
            .ok_or_else(|| AppError::AuthError("Unauthorized".to_string()))?;

        Ok(AuthUser { claims })
    }
}
