mod configuration;
mod errors;
mod middleware;
mod models;
mod repositories;
mod routes;
mod schedulers;
mod services;
mod utils;

use configuration::AppConfig;
use repositories::{MongoProductRepository, MongoUserRepository, UserRepository};
use services::{AuthService, ProductService};
use std::{net::SocketAddr, sync::Arc};
use utils::{db::connect, jwt::JwtUtils, logger::startLogger};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ── Bootstrap ────────────────────────────────────────────────────────────
    dotenv::dotenv().ok();
    startLogger();

    // ── Configuration ────────────────────────────────────────────────────────
    let config = AppConfig::from_env().expect("Failed to load configuration");
    log::info!("Configuration loaded successfully");

    // ── Database ─────────────────────────────────────────────────────────────
    let db = connect(&config).await?;
    log::info!("Database connected");

    // ── Repositories ─────────────────────────────────────────────────────────
    let user_repo = Arc::new(MongoUserRepository::new(&db));
    let product_repo = Arc::new(MongoProductRepository::new(&db));

    // ── Indexes ──────────────────────────────────────────────────────────────
    user_repo.create_indexes().await?;
    product_repo.create_indexes().await?;
    log::info!("Database indexes created");

    // ── Utilities ────────────────────────────────────────────────────────────
    let jwt_utils = Arc::new(JwtUtils::new(&config));

    // ── Services ─────────────────────────────────────────────────────────────
    let auth_service = Arc::new(AuthService::new(user_repo.clone(), jwt_utils.clone()));
    let product_service = Arc::new(ProductService::new(product_repo.clone()));

    // ── Seed check (first run → admin account reminder) ──────────────────────
    match user_repo.count().await {
        Ok(0) => {
            log::info!("No users found. Creating initial admin user...");
            log::info!(
                "Admin credentials - Email: {}, Password: {}",
                config.admin_email,
                config.admin_password
            );
            log::warn!("⚠️  IMPORTANT: Change the admin password after first login!");
        }
        Ok(count) => log::info!("Found {} existing user(s)", count),
        Err(e) => log::error!("Failed to count users: {}", e),
    }

    // ── Routes ───────────────────────────────────────────────────────────────
    let app = routes::create_routes(auth_service, product_service, jwt_utils);

    log::info!("📝 API routes registered:");
    log::info!("   Auth:");
    log::info!("   - POST   /api/auth/register       - Register new user");
    log::info!("   - POST   /api/auth/login          - Login");
    log::info!("   - GET    /api/auth/me             - Get current user (protected)");
    log::info!("   - GET    /api/auth/health         - Health check");
    log::info!("   Users (JWT required):");
    log::info!("   - POST   /api/users               - Create user (Admin)");
    log::info!("   - GET    /api/users               - List users (Admin/Manager)");
    log::info!("   - GET    /api/users/:id           - Get user (Admin/self)");
    log::info!("   - PUT    /api/users/:id           - Update user (Admin/self)");
    log::info!("   - PATCH  /api/users/change-password - Change own password");
    log::info!("   - PATCH  /api/users/:id/role      - Change user role (Admin)");
    log::info!("   - PATCH  /api/users/:id/active    - Activate/deactivate (Admin)");
    log::info!("   - DELETE /api/users/:id           - Delete user (Admin)");
    log::info!("   Products (JWT required):");
    log::info!("   - POST   /api/products            - Create product (protected)");
    log::info!("   - GET    /api/products            - List products (protected)");
    log::info!("   - GET    /api/products/:id        - Get product (protected)");
    log::info!("   - PUT    /api/products/:id        - Update product (protected)");
    log::info!("   - DELETE /api/products/:id        - Delete product (Admin only)");
    log::info!("   - PATCH  /api/products/:id/stock  - Update stock (protected)");
    log::info!("   Alerts:");
    log::info!("   - GET    /api/products/alerts/low-stock  - Low stock products");
    log::info!("   - GET    /api/products/alerts/expiring   - Expiring products");
    log::info!("   - GET    /api/products/alerts/expired    - Expired products");

    // ── Server ───────────────────────────────────────────────────────────────
    let addr = config.server_url();
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    log::info!("🚀 Server listening on http://{}", addr);

    // into_make_service_with_connect_info injects the peer SocketAddr into
    // request extensions, which the per-IP rate-limiter middleware requires.
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}