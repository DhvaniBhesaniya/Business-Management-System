//! User management route handlers.
//!
//! All routes are JWT-protected. Admin-only endpoints enforce role checks
//! inside the service layer.
//!
//! Routes (all nested under `/api/users`):
//!
//! | Method | Path                         | Who        | Description                    |
//! |--------|------------------------------|------------|--------------------------------|
//! | POST   | /                            | Admin      | Create a new user with a role  |
//! | GET    | /                            | Admin/Mgr  | List users (paginated)         |
//! | GET    | /:id                         | Admin/self | Get a single user              |
//! | PUT    | /:id                         | Admin/self | Update name / email            |
//! | PATCH  | /change-password             | Self       | Change own password            |
//! | PATCH  | /:id/role                    | Admin      | Change a user's role           |
//! | PATCH  | /:id/active                  | Admin      | Activate / deactivate user     |
//! | DELETE | /:id                         | Admin      | Hard-delete a user             |

use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use crate::{
    errors::AppError,
    models::{
        Claims, ChangePasswordRequest, CreateUserRequest, SetUserActiveRequest,
        UpdateUserRequest, UpdateUserRoleRequest, UserQueryParams,
    },
    services::AuthService,
};

pub struct UserHandler;

impl UserHandler {
    /// POST /api/users
    /// Admin creates a new user with an explicit role.
    pub async fn create(
        State(auth_service): State<Arc<AuthService>>,
        Extension(claims): Extension<Claims>,
        Json(req): Json<CreateUserRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let user = auth_service
            .create_user_by_admin(req, &claims.role)
            .await?;
        Ok((StatusCode::CREATED, Json(user)))
    }

    /// GET /api/users
    /// Admin / Manager lists all users with optional pagination and filters.
    pub async fn list(
        State(auth_service): State<Arc<AuthService>>,
        Extension(claims): Extension<Claims>,
        Query(params): Query<UserQueryParams>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = auth_service.list_users(params, &claims.role).await?;
        Ok(Json(response))
    }

    /// GET /api/users/:id
    /// Admin can fetch any user; regular users can only fetch themselves.
    pub async fn get(
        State(auth_service): State<Arc<AuthService>>,
        Extension(claims): Extension<Claims>,
        Path(id): Path<String>,
    ) -> Result<impl IntoResponse, AppError> {
        // Non-admins may only access their own profile via this route.
        use crate::models::UserRole;
        if claims.role != UserRole::Admin && claims.sub != id {
            return Err(AppError::Forbidden(
                "You can only view your own profile".to_string(),
            ));
        }
        let user = auth_service.get_user_by_id(&id).await?;
        Ok(Json(user))
    }

    /// PUT /api/users/:id
    /// Update a user's name and/or email. Admin can update any user; others can
    /// only update themselves.
    pub async fn update(
        State(auth_service): State<Arc<AuthService>>,
        Extension(claims): Extension<Claims>,
        Path(id): Path<String>,
        Json(req): Json<UpdateUserRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let user = auth_service
            .update_user(&id, req, &claims.sub, &claims.role)
            .await?;
        Ok(Json(user))
    }

    /// PATCH /api/users/change-password
    /// Authenticated user changes their own password.
    /// NOTE: this is a static route and must be registered BEFORE `/:id` in
    /// the router so that `matchit` does not treat "change-password" as an id.
    pub async fn change_password(
        State(auth_service): State<Arc<AuthService>>,
        Extension(claims): Extension<Claims>,
        Json(req): Json<ChangePasswordRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        auth_service.change_password(&claims.sub, req).await?;
        Ok(StatusCode::NO_CONTENT)
    }

    /// PATCH /api/users/:id/role
    /// Admin changes a user's role.
    pub async fn update_role(
        State(auth_service): State<Arc<AuthService>>,
        Extension(claims): Extension<Claims>,
        Path(id): Path<String>,
        Json(req): Json<UpdateUserRoleRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let user = auth_service
            .update_user_role(&id, req, &claims.role)
            .await?;
        Ok(Json(user))
    }

    /// PATCH /api/users/:id/active
    /// Admin activates or deactivates a user's account.
    pub async fn set_active(
        State(auth_service): State<Arc<AuthService>>,
        Extension(claims): Extension<Claims>,
        Path(id): Path<String>,
        Json(req): Json<SetUserActiveRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let user = auth_service.set_user_active(&id, req, &claims.role).await?;
        Ok(Json(user))
    }

    /// DELETE /api/users/:id
    /// Admin hard-deletes a user. An admin cannot delete themselves.
    pub async fn delete(
        State(auth_service): State<Arc<AuthService>>,
        Extension(claims): Extension<Claims>,
        Path(id): Path<String>,
    ) -> Result<impl IntoResponse, AppError> {
        auth_service
            .delete_user(&id, &claims.sub, &claims.role)
            .await?;
        Ok(StatusCode::NO_CONTENT)
    }
}
