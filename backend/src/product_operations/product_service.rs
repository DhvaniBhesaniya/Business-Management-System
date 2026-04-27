use std::sync::Arc;
use validator::Validate;

use crate::errors::AppError;
use crate::user_operations::UserRole;
use super::product_structure::{
    CreateProductRequest, Product, ProductListResponse, ProductQueryParams,
    ProductRepository, ProductResponse, UpdateProductRequest,
};

#[derive(Clone)]
pub struct ProductService {
    product_repo: Arc<dyn ProductRepository>,
}

impl ProductService {
    pub fn new(product_repo: Arc<dyn ProductRepository>) -> Self {
        Self { product_repo }
    }

    pub async fn create_product(
        &self,
        req: CreateProductRequest,
        created_by: String,
        user_role: &UserRole,
    ) -> Result<ProductResponse, AppError> {
        // Authorization check
        if !self.can_manage_products(user_role) {
            return Err(AppError::Forbidden(
                "You don't have permission to create products".to_string(),
            ));
        }

        // Validate input
        req.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        // Business validation
        if req.selling_price < req.purchase_price {
            return Err(AppError::ValidationError(
                "Selling price cannot be less than purchase price".to_string(),
            ));
        }

        if req.mrp < req.selling_price {
            return Err(AppError::ValidationError(
                "MRP cannot be less than selling price".to_string(),
            ));
        }

        if req.has_expiry && req.expiry_date.is_none() {
            return Err(AppError::ValidationError(
                "Expiry date is required when product has expiry".to_string(),
            ));
        }

        // Create product
        let mut product = Product::new(
            req.name,
            req.category,
            req.purchase_price,
            req.selling_price,
            req.mrp,
            req.gst_rate,
            req.stock_quantity,
            req.unit,
            created_by,
        );

        // Set optional fields
        product.brand = req.brand;
        product.description = req.description;
        product.min_stock_level = req.min_stock_level.unwrap_or(10.0);
        product.sku = req.sku;
        product.barcode = req.barcode;
        product.has_expiry = req.has_expiry;
        product.manufacturing_date = req.manufacturing_date;
        product.expiry_date = req.expiry_date;
        product.batch_number = req.batch_number;
        product.supplier_name = req.supplier_name;
        product.supplier_contact = req.supplier_contact;

        let created_product = self.product_repo.create(product).await?;

        Ok(ProductResponse::from(created_product))
    }

    pub async fn get_product(&self, product_id: &str) -> Result<ProductResponse, AppError> {
        let product = self
            .product_repo
            .find_by_id(product_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        Ok(ProductResponse::from(product))
    }

    pub async fn list_products(
        &self,
        params: ProductQueryParams,
    ) -> Result<ProductListResponse, AppError> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20).min(100);

        let (products, total) = self.product_repo.find_all(params).await?;

        let product_responses: Vec<ProductResponse> = products
            .into_iter()
            .map(ProductResponse::from)
            .collect();

        let total_pages = (total as f64 / limit as f64).ceil() as u64;

        Ok(ProductListResponse {
            products: product_responses,
            total,
            page,
            limit,
            total_pages,
        })
    }

    pub async fn update_product(
        &self,
        product_id: &str,
        update: UpdateProductRequest,
        user_role: &UserRole,
    ) -> Result<ProductResponse, AppError> {
        // Authorization check
        if !self.can_manage_products(user_role) {
            return Err(AppError::Forbidden(
                "You don't have permission to update products".to_string(),
            ));
        }

        // Business validations
        if let (Some(selling), Some(purchase)) = (update.selling_price, update.purchase_price) {
            if selling < purchase {
                return Err(AppError::ValidationError(
                    "Selling price cannot be less than purchase price".to_string(),
                ));
            }
        }

        if let (Some(mrp), Some(selling)) = (update.mrp, update.selling_price) {
            if mrp < selling {
                return Err(AppError::ValidationError(
                    "MRP cannot be less than selling price".to_string(),
                ));
            }
        }

        let updated_product = self.product_repo.update(product_id, update).await?;

        Ok(ProductResponse::from(updated_product))
    }

    pub async fn delete_product(
        &self,
        product_id: &str,
        user_role: &UserRole,
    ) -> Result<(), AppError> {
        // Authorization check - only admins can delete
        if *user_role != UserRole::Admin {
            return Err(AppError::Forbidden(
                "Only admins can delete products".to_string(),
            ));
        }

        self.product_repo.delete(product_id).await
    }

    pub async fn update_stock(
        &self,
        product_id: &str,
        quantity_change: f64,
        user_role: &UserRole,
    ) -> Result<ProductResponse, AppError> {
        // Authorization check
        if !self.can_manage_products(user_role) {
            return Err(AppError::Forbidden(
                "You don't have permission to update stock".to_string(),
            ));
        }

        let updated_product = self
            .product_repo
            .update_stock(product_id, quantity_change)
            .await?;

        // Check if stock is negative
        if updated_product.stock_quantity < 0.0 {
            return Err(AppError::ValidationError(
                "Stock quantity cannot be negative".to_string(),
            ));
        }

        Ok(ProductResponse::from(updated_product))
    }

    pub async fn get_low_stock_products(&self) -> Result<Vec<ProductResponse>, AppError> {
        let products = self.product_repo.get_low_stock_products().await?;

        Ok(products
            .into_iter()
            .map(ProductResponse::from)
            .collect())
    }

    pub async fn get_expiring_products(
        &self,
        days: i64,
    ) -> Result<Vec<ProductResponse>, AppError> {
        let products = self.product_repo.get_expiring_products(days).await?;

        Ok(products
            .into_iter()
            .map(ProductResponse::from)
            .collect())
    }

    pub async fn get_expired_products(&self) -> Result<Vec<ProductResponse>, AppError> {
        let products = self.product_repo.get_expired_products().await?;

        Ok(products
            .into_iter()
            .map(ProductResponse::from)
            .collect())
    }

    // Helper methods
    fn can_manage_products(&self, role: &UserRole) -> bool {
        matches!(role, UserRole::Admin | UserRole::Manager)
    }
}
