use crate::{
    middleware::RateLimiter,
    product_operations::ProductService,
    user_operations::UserService,
    utils::JwtUtils,
};
use axum::{middleware, Router};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub fn create_routes(
    user_service: Arc<UserService>,
    product_service: Arc<ProductService>,
    jwt_utils: Arc<JwtUtils>,
) -> Router {
    // Rate limiter: 100 requests per 60-second window per IP address
    let rate_limiter = RateLimiter::new(100, 60);

    // Delegate route construction to each operations module
    let (auth_routes, user_routes) =
        crate::user_operations::routes(user_service, jwt_utils.clone());
    let product_routes = crate::product_operations::routes(product_service, jwt_utils);

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
