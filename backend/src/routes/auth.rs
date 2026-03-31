use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use crate::{
    errors::AppError,
    models::{Claims, LoginRequest, RegisterRequest},
    services::AuthService,
};

pub struct AuthHandler;

impl AuthHandler {
    /// GET /api/auth/health
    /// Health check endpoint – no authentication required.
    pub async fn health() -> impl IntoResponse {
        Json(serde_json::json!({
            "status": "ok",
            "message": "shree-nandi-backend is running"
        }))
    }

    /// POST /api/auth/register
    /// Register a new user (only works when zero users exist – first user becomes admin).
    pub async fn register(
        State(auth_service): State<Arc<AuthService>>,
        Json(req): Json<RegisterRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = auth_service.register(req).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }

    /// POST /api/auth/login
    /// Authenticate an existing user and return a JWT.
    pub async fn login(
        State(auth_service): State<Arc<AuthService>>,
        Json(req): Json<LoginRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = auth_service.login(req).await?;
        Ok(Json(response))
    }

    /// GET /api/auth/me
    /// Return the currently authenticated user's profile.
    pub async fn me(
        State(auth_service): State<Arc<AuthService>>,
        Extension(claims): Extension<Claims>,
    ) -> Result<impl IntoResponse, AppError> {
        let user = auth_service.get_user_by_id(&claims.sub).await?;
        Ok(Json(user))
    }
}
