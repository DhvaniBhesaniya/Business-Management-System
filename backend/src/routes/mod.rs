pub mod auth;
pub mod product;
pub mod user;

use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use crate::{
    middleware::{AuthMiddleware, RateLimiter},
    services::{AuthService, ProductService},
    utils::JwtUtils,
};
use auth::AuthHandler;
use product::ProductHandler;
use user::UserHandler;

pub fn create_routes(
    auth_service: Arc<AuthService>,
    product_service: Arc<ProductService>,
    jwt_utils: Arc<JwtUtils>,
) -> Router {
    // Rate limiter: 100 requests per 60-second window per IP address
    let rate_limiter = RateLimiter::new(100, 60);

    // ── Public auth routes (no JWT required) ──────────────────────────────
    let public_auth_routes = Router::new()
        .route("/health", get(AuthHandler::health))
        .route("/register", post(AuthHandler::register))
        .route("/login", post(AuthHandler::login));

    // ── Protected auth routes (JWT required) ────────────────────────────
    let protected_auth_routes = Router::new()
        .route("/me", get(AuthHandler::me))
        .layer(middleware::from_fn_with_state(
            jwt_utils.clone(),
            AuthMiddleware::auth,
        ));

    let auth_routes = Router::new()
        .merge(public_auth_routes)
        .merge(protected_auth_routes)
        .with_state(auth_service.clone());

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
            jwt_utils.clone(),
            AuthMiddleware::auth,
        ))
        .with_state(auth_service);

    // ── Product routes (all JWT-protected) ──────────────────────────────
    // NOTE: Static-path alert routes MUST be declared before the `:id`
    // wildcard routes so that `matchit` resolves them correctly.
    let product_routes = Router::new()
        .route("/products/alerts/low-stock", get(ProductHandler::low_stock))
        .route("/products/alerts/expiring", get(ProductHandler::expiring_soon))
        .route("/products/alerts/expired", get(ProductHandler::expired))
        .route("/products", post(ProductHandler::create))
        .route("/products", get(ProductHandler::list))
        .route("/products/:id", get(ProductHandler::get))
        .route("/products/:id", put(ProductHandler::update))
        .route("/products/:id", delete(ProductHandler::delete))
        .route("/products/:id/stock", patch(ProductHandler::update_stock))
        .layer(middleware::from_fn_with_state(
            jwt_utils.clone(),
            AuthMiddleware::auth,
        ))
        .with_state(product_service);

    // ── Compose the full router with CORS + rate limiting ─────────────────
    Router::new()
        .nest("/api/auth", auth_routes)
        .nest("/api/users", user_routes)
        .nest("/api", product_routes)
        .layer(middleware::from_fn_with_state(
            rate_limiter,
            RateLimiter::rate_limit,
        ))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
}

