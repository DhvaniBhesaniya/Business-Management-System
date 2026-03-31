use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub category: String,
    pub brand: Option<String>,
    pub description: Option<String>,
    
    // Pricing
    pub purchase_price: f64,
    pub selling_price: f64,
    pub mrp: f64,
    pub gst_rate: f64, // GST percentage (e.g., 5, 12, 18)
    
    // Inventory
    pub stock_quantity: f64,
    pub unit: String, // kg, pieces, packets, liters, etc.
    pub min_stock_level: f64, // Alert when stock goes below this
    pub sku: Option<String>, // Stock Keeping Unit
    pub barcode: Option<String>,
    
    // Expiry Management
    pub has_expiry: bool,
    pub manufacturing_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub batch_number: Option<String>,
    
    // Supplier
    pub supplier_name: Option<String>,
    pub supplier_contact: Option<String>,
    
    // Media
    pub images: Vec<String>, // URLs or paths to images
    
    // Metadata
    pub is_active: bool,
    pub created_by: String, // User ID who created
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Product {
    pub fn new(
        name: String,
        category: String,
        purchase_price: f64,
        selling_price: f64,
        mrp: f64,
        gst_rate: f64,
        stock_quantity: f64,
        unit: String,
        created_by: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            name,
            category,
            brand: None,
            description: None,
            purchase_price,
            selling_price,
            mrp,
            gst_rate,
            stock_quantity,
            unit,
            min_stock_level: 10.0, // Default
            sku: None,
            barcode: None,
            has_expiry: false,
            manufacturing_date: None,
            expiry_date: None,
            batch_number: None,
            supplier_name: None,
            supplier_contact: None,
            images: Vec::new(),
            is_active: true,
            created_by,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_low_stock(&self) -> bool {
        self.stock_quantity <= self.min_stock_level
    }

    pub fn is_expired(&self) -> bool {
        if !self.has_expiry {
            return false;
        }
        
        if let Some(expiry) = self.expiry_date {
            return Utc::now() > expiry;
        }
        
        false
    }

    pub fn days_until_expiry(&self) -> Option<i64> {
        if !self.has_expiry {
            return None;
        }
        
        self.expiry_date.map(|expiry| {
            let duration = expiry.signed_duration_since(Utc::now());
            duration.num_days()
        })
    }

    pub fn profit_per_unit(&self) -> f64 {
        self.selling_price - self.purchase_price
    }

    pub fn profit_percentage(&self) -> f64 {
        if self.purchase_price == 0.0 {
            return 0.0;
        }
        ((self.selling_price - self.purchase_price) / self.purchase_price) * 100.0
    }
}

// DTOs (Data Transfer Objects)

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 2, message = "Product name must be at least 2 characters"))]
    pub name: String,
    
    #[validate(length(min = 2, message = "Category must be at least 2 characters"))]
    pub category: String,
    
    pub brand: Option<String>,
    pub description: Option<String>,
    
    #[validate(range(min = 0.0, message = "Purchase price must be positive"))]
    pub purchase_price: f64,
    
    #[validate(range(min = 0.0, message = "Selling price must be positive"))]
    pub selling_price: f64,
    
    #[validate(range(min = 0.0, message = "MRP must be positive"))]
    pub mrp: f64,
    
    #[validate(range(min = 0.0, max = 100.0, message = "GST rate must be between 0 and 100"))]
    pub gst_rate: f64,
    
    #[validate(range(min = 0.0, message = "Stock quantity must be positive"))]
    pub stock_quantity: f64,
    
    #[validate(length(min = 1, message = "Unit is required"))]
    pub unit: String,
    
    pub min_stock_level: Option<f64>,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    
    pub has_expiry: bool,
    pub manufacturing_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub batch_number: Option<String>,
    
    pub supplier_name: Option<String>,
    pub supplier_contact: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub category: Option<String>,
    pub brand: Option<String>,
    pub description: Option<String>,
    pub purchase_price: Option<f64>,
    pub selling_price: Option<f64>,
    pub mrp: Option<f64>,
    pub gst_rate: Option<f64>,
    pub stock_quantity: Option<f64>,
    pub unit: Option<String>,
    pub min_stock_level: Option<f64>,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub has_expiry: Option<bool>,
    pub manufacturing_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub batch_number: Option<String>,
    pub supplier_name: Option<String>,
    pub supplier_contact: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ProductQueryParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub category: Option<String>,
    pub search: Option<String>, // Search in name, brand, description
    pub low_stock: Option<bool>, // Filter low stock items
    pub expiring_soon: Option<bool>, // Filter items expiring in 30 days
    pub expired: Option<bool>, // Filter expired items
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: String,
    pub name: String,
    pub category: String,
    pub brand: Option<String>,
    pub description: Option<String>,
    pub purchase_price: f64,
    pub selling_price: f64,
    pub mrp: f64,
    pub gst_rate: f64,
    pub stock_quantity: f64,
    pub unit: String,
    pub min_stock_level: f64,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub has_expiry: bool,
    pub manufacturing_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub batch_number: Option<String>,
    pub supplier_name: Option<String>,
    pub supplier_contact: Option<String>,
    pub images: Vec<String>,
    pub is_active: bool,
    pub is_low_stock: bool,
    pub is_expired: bool,
    pub days_until_expiry: Option<i64>,
    pub profit_per_unit: f64,
    pub profit_percentage: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        Self {
            id: product.id.unwrap().to_hex(),
            name: product.name.clone(),
            category: product.category.clone(),
            brand: product.brand.clone(),
            description: product.description.clone(),
            purchase_price: product.purchase_price,
            selling_price: product.selling_price,
            mrp: product.mrp,
            gst_rate: product.gst_rate,
            stock_quantity: product.stock_quantity,
            unit: product.unit.clone(),
            min_stock_level: product.min_stock_level,
            sku: product.sku.clone(),
            barcode: product.barcode.clone(),
            has_expiry: product.has_expiry,
            manufacturing_date: product.manufacturing_date,
            expiry_date: product.expiry_date,
            batch_number: product.batch_number.clone(),
            supplier_name: product.supplier_name.clone(),
            supplier_contact: product.supplier_contact.clone(),
            images: product.images.clone(),
            is_active: product.is_active,
            is_low_stock: product.is_low_stock(),
            is_expired: product.is_expired(),
            days_until_expiry: product.days_until_expiry(),
            profit_per_unit: product.profit_per_unit(),
            profit_percentage: product.profit_percentage(),
            created_at: product.created_at,
            updated_at: product.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProductListResponse {
    pub products: Vec<ProductResponse>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
    pub total_pages: u64,
}