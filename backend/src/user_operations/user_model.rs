//! User and authentication route handlers.
//!
//! All auth handlers are in `AuthHandler`, all user-management handlers in
//! `UserHandler`. Both operate on `UserService`.

use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use super::user_service::UserService;
use super::user_structure::{
    ChangePasswordRequest, Claims, CreateUserRequest, LoginRequest, RegisterRequest,
    SetUserActiveRequest, UpdateUserRequest, UpdateUserRoleRequest, UserQueryParams, UserRole,
};
use crate::errors::AppError;

// ── Auth Handlers ────────────────────────────────────────────────────────────

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
        State(user_service): State<Arc<UserService>>,
        Json(req): Json<RegisterRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = user_service.register(req).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }

    /// POST /api/auth/login
    /// Authenticate an existing user and return a JWT.
    pub async fn login(
        State(user_service): State<Arc<UserService>>,
        Json(req): Json<LoginRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = user_service.login(req).await?;
        Ok(Json(response))
    }

    /// GET /api/auth/me
    /// Return the currently authenticated user's profile.
    pub async fn me(
        State(user_service): State<Arc<UserService>>,
        Extension(claims): Extension<Claims>,
    ) -> Result<impl IntoResponse, AppError> {
        let user = user_service.get_user_by_id(&claims.sub).await?;
        Ok(Json(user))
    }
}

// ── User Management Handlers ─────────────────────────────────────────────────

pub struct UserHandler;

impl UserHandler {
    /// POST /api/users
    /// Admin creates a new user with an explicit role.
    pub async fn create(
        State(user_service): State<Arc<UserService>>,
        Extension(claims): Extension<Claims>,
        Json(req): Json<CreateUserRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let user = user_service.create_user_by_admin(req, &claims.role).await?;
        Ok((StatusCode::CREATED, Json(user)))
    }

    /// GET /api/users
    /// Admin / Manager lists all users with optional pagination and filters.
    pub async fn list(
        State(user_service): State<Arc<UserService>>,
        Extension(claims): Extension<Claims>,
        Query(params): Query<UserQueryParams>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = user_service.list_users(params, &claims.role).await?;
        Ok(Json(response))
    }

    /// GET /api/users/:id
    /// Admin can fetch any user; regular users can only fetch themselves.
    pub async fn get(
        State(user_service): State<Arc<UserService>>,
        Extension(claims): Extension<Claims>,
        Path(id): Path<String>,
    ) -> Result<impl IntoResponse, AppError> {
        // Non-admins may only access their own profile via this route.
        if claims.role != UserRole::Admin && claims.sub != id {
            return Err(AppError::Forbidden(
                "You can only view your own profile".to_string(),
            ));
        }
        let user = user_service.get_user_by_id(&id).await?;
        Ok(Json(user))
    }

    /// PUT /api/users/:id
    /// Update a user's name and/or email. Admin can update any user; others can
    /// only update themselves.
    pub async fn update(
        State(user_service): State<Arc<UserService>>,
        Extension(claims): Extension<Claims>,
        Path(id): Path<String>,
        Json(req): Json<UpdateUserRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let user = user_service
            .update_user(&id, req, &claims.sub, &claims.role)
            .await?;
        Ok(Json(user))
    }

    /// PATCH /api/users/change-password
    /// Authenticated user changes their own password.
    /// NOTE: this is a static route and must be registered BEFORE `/:id` in
    /// the router so that `matchit` does not treat "change-password" as an id.
    pub async fn change_password(
        State(user_service): State<Arc<UserService>>,
        Extension(claims): Extension<Claims>,
        Json(req): Json<ChangePasswordRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        user_service.change_password(&claims.sub, req).await?;
        Ok(StatusCode::NO_CONTENT)
    }

    /// PATCH /api/users/:id/role
    /// Admin changes a user's role.
    pub async fn update_role(
        State(user_service): State<Arc<UserService>>,
        Extension(claims): Extension<Claims>,
        Path(id): Path<String>,
        Json(req): Json<UpdateUserRoleRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let user = user_service
            .update_user_role(&id, req, &claims.role)
            .await?;
        Ok(Json(user))
    }

    /// PATCH /api/users/:id/active
    /// Admin activates or deactivates a user's account.
    pub async fn set_active(
        State(user_service): State<Arc<UserService>>,
        Extension(claims): Extension<Claims>,
        Path(id): Path<String>,
        Json(req): Json<SetUserActiveRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let user = user_service.set_user_active(&id, req, &claims.role).await?;
        Ok(Json(user))
    }

    /// DELETE /api/users/:id
    /// Admin hard-deletes a user. An admin cannot delete themselves.
    pub async fn delete(
        State(user_service): State<Arc<UserService>>,
        Extension(claims): Extension<Claims>,
        Path(id): Path<String>,
    ) -> Result<impl IntoResponse, AppError> {
        user_service
            .delete_user(&id, &claims.sub, &claims.role)
            .await?;
        Ok(StatusCode::NO_CONTENT)
    }
}
