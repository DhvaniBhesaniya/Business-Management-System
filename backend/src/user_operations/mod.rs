pub mod user_model;
pub mod user_service;
pub mod user_structure;

// ── Re-exports for external use ──────────────────────────────────────────────

pub use user_structure::{
    Claims, MongoUserRepository, User, UserRepository, UserRole,
};
pub use user_service::UserService;

// ── Route construction ───────────────────────────────────────────────────────

use crate::middleware::AuthMiddleware;
use crate::utils::JwtUtils;
use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};
use std::sync::Arc;
use user_model::{AuthHandler, UserHandler};

/// Returns `(auth_routes, user_management_routes)` ready for nesting.
pub fn routes(user_service: Arc<UserService>, jwt_utils: Arc<JwtUtils>) -> (Router, Router) {
    // ── Public auth routes (no JWT required) ──────────────────────────────
    let public_auth = Router::new()
        .route("/health", get(AuthHandler::health))
        .route("/register", post(AuthHandler::register))
        .route("/login", post(AuthHandler::login));

    // ── Protected auth routes (JWT required) ──────────────────────────────
    let protected_auth = Router::new()
        .route("/me", get(AuthHandler::me))
        .layer(middleware::from_fn_with_state(
            jwt_utils.clone(),
            AuthMiddleware::auth,
        ));

    let auth_routes = Router::new()
        .merge(public_auth)
        .merge(protected_auth)
        .with_state(user_service.clone());

    // ── User management routes (all JWT-protected) ────────────────────────
    // Static segments MUST be registered before `/:id` so that matchit
    // does not treat e.g. "change-password" as an id value.
    let user_routes = Router::new()
        .route("/change-password", patch(UserHandler::change_password))
        .route("/", post(UserHandler::create))
        .route("/", get(UserHandler::list))
        .route("/:id", get(UserHandler::get))
        .route("/:id", put(UserHandler::update))
        .route("/:id/role", patch(UserHandler::update_role))
        .route("/:id/active", patch(UserHandler::set_active))
        .route("/:id", delete(UserHandler::delete))
        .layer(middleware::from_fn_with_state(
            jwt_utils,
            AuthMiddleware::auth,
        ))
        .with_state(user_service);

    (auth_routes, user_routes)
}
