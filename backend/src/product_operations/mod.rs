pub mod product_model;
pub mod product_service;
pub mod product_structure;

// ── Re-exports for external use ──────────────────────────────────────────────

pub use product_structure::{MongoProductRepository, ProductRepository};
pub use product_service::ProductService;

// ── Route construction ───────────────────────────────────────────────────────

use crate::middleware::AuthMiddleware;
use crate::utils::JwtUtils;
use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};
use std::sync::Arc;
use product_model::ProductHandler;

/// Returns a `Router` for all product endpoints, ready for nesting.
pub fn routes(product_service: Arc<ProductService>, jwt_utils: Arc<JwtUtils>) -> Router {
    // NOTE: Static-path alert routes MUST be declared before the `:id`
    // wildcard routes so that `matchit` resolves them correctly.
    Router::new()
        .route("/products/alerts/low-stock", get(ProductHandler::low_stock))
        .route(
            "/products/alerts/expiring",
            get(ProductHandler::expiring_soon),
        )
        .route("/products/alerts/expired", get(ProductHandler::expired))
        .route("/products", post(ProductHandler::create))
        .route("/products", get(ProductHandler::list))
        .route("/products/:id", get(ProductHandler::get))
        .route("/products/:id", put(ProductHandler::update))
        .route("/products/:id", delete(ProductHandler::delete))
        .route("/products/:id/stock", patch(ProductHandler::update_stock))
        .layer(middleware::from_fn_with_state(
            jwt_utils,
            AuthMiddleware::auth,
        ))
        .with_state(product_service)
}
