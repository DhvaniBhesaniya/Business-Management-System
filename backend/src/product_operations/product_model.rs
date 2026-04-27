use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::errors::AppError;
use crate::user_operations::Claims;
use super::product_structure::{CreateProductRequest, ProductQueryParams, UpdateProductRequest};
use super::product_service::ProductService;

// ── DTOs used only by this handler module ────────────────────────────────────

/// Request body for PATCH /api/products/:id/stock
#[derive(Debug, Deserialize)]
pub struct UpdateStockRequest {
    /// Positive value adds stock; negative value subtracts stock.
    pub quantity_change: f64,
}

/// Query parameters for GET /api/products/alerts/expiring
#[derive(Debug, Deserialize)]
pub struct ExpiringQueryParams {
    /// How many days ahead to look. Defaults to 30.
    pub days: Option<i64>,
}

// ── Handler struct ───────────────────────────────────────────────────────────

pub struct ProductHandler;

impl ProductHandler {
    /// POST /api/products
    /// Create a new product (Admin / Manager only).
    pub async fn create(
        State(product_service): State<Arc<ProductService>>,
        Extension(claims): Extension<Claims>,
        Json(create_req): Json<CreateProductRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let product = product_service
            .create_product(create_req, claims.sub.clone(), &claims.role)
            .await?;
        Ok((StatusCode::CREATED, Json(product)))
    }

    /// GET /api/products/:id
    /// Fetch a single product by its MongoDB ObjectId.
    pub async fn get(
        State(product_service): State<Arc<ProductService>>,
        Path(id): Path<String>,
    ) -> Result<impl IntoResponse, AppError> {
        let product = product_service.get_product(&id).await?;
        Ok(Json(product))
    }

    /// GET /api/products
    /// Paginated product list with optional filters.
    pub async fn list(
        State(product_service): State<Arc<ProductService>>,
        Query(params): Query<ProductQueryParams>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = product_service.list_products(params).await?;
        Ok(Json(response))
    }

    /// PUT /api/products/:id
    /// Update a product (Admin / Manager only).
    pub async fn update(
        State(product_service): State<Arc<ProductService>>,
        Path(id): Path<String>,
        Extension(claims): Extension<Claims>,
        Json(update_req): Json<UpdateProductRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let product = product_service
            .update_product(&id, update_req, &claims.role)
            .await?;
        Ok(Json(product))
    }

    /// DELETE /api/products/:id
    /// Hard-delete a product (Admin only).
    pub async fn delete(
        State(product_service): State<Arc<ProductService>>,
        Path(id): Path<String>,
        Extension(claims): Extension<Claims>,
    ) -> Result<impl IntoResponse, AppError> {
        product_service.delete_product(&id, &claims.role).await?;
        Ok(StatusCode::NO_CONTENT)
    }

    /// PATCH /api/products/:id/stock
    /// Atomically increment or decrement stock quantity.
    pub async fn update_stock(
        State(product_service): State<Arc<ProductService>>,
        Path(id): Path<String>,
        Extension(claims): Extension<Claims>,
        Json(stock_req): Json<UpdateStockRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let product = product_service
            .update_stock(&id, stock_req.quantity_change, &claims.role)
            .await?;
        Ok(Json(product))
    }

    /// GET /api/products/alerts/low-stock
    /// Products whose stock is at or below their minimum stock level.
    pub async fn low_stock(
        State(product_service): State<Arc<ProductService>>,
    ) -> Result<impl IntoResponse, AppError> {
        let products = product_service.get_low_stock_products().await?;
        Ok(Json(products))
    }

    /// GET /api/products/alerts/expiring[?days=N]
    /// Products that expire within the next N days (default: 30).
    pub async fn expiring_soon(
        State(product_service): State<Arc<ProductService>>,
        Query(params): Query<ExpiringQueryParams>,
    ) -> Result<impl IntoResponse, AppError> {
        let days = params.days.unwrap_or(30);
        let products = product_service.get_expiring_products(days).await?;
        Ok(Json(products))
    }

    /// GET /api/products/alerts/expired
    /// All products whose expiry date is in the past.
    pub async fn expired(
        State(product_service): State<Arc<ProductService>>,
    ) -> Result<impl IntoResponse, AppError> {
        let products = product_service.get_expired_products().await?;
        Ok(Json(products))
    }
}
