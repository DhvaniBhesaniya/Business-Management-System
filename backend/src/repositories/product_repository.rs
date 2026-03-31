use async_trait::async_trait;
use chrono::Utc;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::{FindOptions, IndexOptions},
    Collection, Database, IndexModel,
};
use crate::{
    errors::AppError,
    models::{Product, ProductQueryParams, UpdateProductRequest},
};

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn create(&self, product: Product) -> Result<Product, AppError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Product>, AppError>;
    async fn find_all(&self, params: ProductQueryParams) -> Result<(Vec<Product>, u64), AppError>;
    async fn update(&self, id: &str, update: UpdateProductRequest) -> Result<Product, AppError>;
    async fn delete(&self, id: &str) -> Result<(), AppError>;
    async fn update_stock(&self, id: &str, quantity_change: f64) -> Result<Product, AppError>;
    async fn get_low_stock_products(&self) -> Result<Vec<Product>, AppError>;
    async fn get_expiring_products(&self, days: i64) -> Result<Vec<Product>, AppError>;
    async fn get_expired_products(&self) -> Result<Vec<Product>, AppError>;
}

#[derive(Clone)]
pub struct MongoProductRepository {
    collection: Collection<Product>,
}

impl MongoProductRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("products"),
        }
    }

    pub async fn create_indexes(&self) -> Result<(), AppError> {
        // Index on name for text search
        let name_index = IndexModel::builder()
            .keys(doc! { "name": 1 })
            .build();

        // Index on category
        let category_index = IndexModel::builder()
            .keys(doc! { "category": 1 })
            .build();

        // Index on expiry date
        let expiry_index = IndexModel::builder()
            .keys(doc! { "expiry_date": 1 })
            .build();

        // Index on stock quantity for low stock queries
        let stock_index = IndexModel::builder()
            .keys(doc! { "stock_quantity": 1 })
            .build();

        // Compound index for search
        let search_index = IndexModel::builder()
            .keys(doc! { "name": "text", "brand": "text", "description": "text" })
            .build();

        // Unique index on SKU (if provided)
        let sku_index = IndexModel::builder()
            .keys(doc! { "sku": 1 })
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .sparse(true) // Allow null values
                    .build(),
            )
            .build();

        self.collection
            .create_indexes(
                vec![
                    name_index,
                    category_index,
                    expiry_index,
                    stock_index,
                    search_index,
                    sku_index,
                ],
                None,
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        log::info!("Product indexes created successfully");
        Ok(())
    }

    fn build_filter(&self, params: &ProductQueryParams) -> Document {
        let mut filter = doc! {};

        if let Some(category) = &params.category {
            filter.insert("category", category.clone());
        }

        if let Some(search) = &params.search {
            filter.insert("$text", doc! { "$search": search.clone() });
        }

        if let Some(is_active) = params.is_active {
            filter.insert("is_active", is_active);
        }

        // Low stock filter
        if let Some(true) = params.low_stock {
            filter.insert("$expr", doc! {
                "$lte": ["$stock_quantity", "$min_stock_level"]
            });
        }

        // Expiring soon filter (within 30 days)
        if let Some(true) = params.expiring_soon {
            let thirty_days_from_now = Utc::now() + chrono::Duration::days(30);
            filter.insert("has_expiry", true);
            filter.insert("expiry_date", doc! {
                "$lte": thirty_days_from_now,
                "$gte": Utc::now()
            });
        }

        // Expired filter
        if let Some(true) = params.expired {
            filter.insert("has_expiry", true);
            filter.insert("expiry_date", doc! {
                "$lt": Utc::now()
            });
        }

        filter
    }
}

#[async_trait]
impl ProductRepository for MongoProductRepository {
    async fn create(&self, mut product: Product) -> Result<Product, AppError> {
        let result = self
            .collection
            .insert_one(&product, None)
            .await
            .map_err(|e| {
                if e.to_string().contains("duplicate key") {
                    AppError::Conflict("Product with this SKU already exists".to_string())
                } else {
                    AppError::DatabaseError(e.to_string())
                }
            })?;

        product.id = Some(result.inserted_id.as_object_id().unwrap());
        Ok(product)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Product>, AppError> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest("Invalid product ID".to_string()))?;

        let product = self
            .collection
            .find_one(doc! { "_id": object_id }, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(product)
    }

    async fn find_all(&self, params: ProductQueryParams) -> Result<(Vec<Product>, u64), AppError> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20).min(100); // Max 100 items per page
        let skip = (page - 1) * limit;

        let filter = self.build_filter(&params);

        // Count total documents
        let total = self
            .collection
            .count_documents(filter.clone(), None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Find documents with pagination
        let find_options = FindOptions::builder()
            .skip(skip)
            .limit(limit as i64)
            .sort(doc! { "created_at": -1 }) // Most recent first
            .build();

        let mut cursor = self
            .collection
            .find(filter, find_options)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut products = Vec::new();
        while cursor.advance().await.map_err(|e| AppError::DatabaseError(e.to_string()))? {
            let product = cursor
                .deserialize_current()
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            products.push(product);
        }

        Ok((products, total))
    }

    async fn update(&self, id: &str, update: UpdateProductRequest) -> Result<Product, AppError> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest("Invalid product ID".to_string()))?;

        // Build update document
        let mut update_doc = doc! {
            "$set": {
                "updated_at": Utc::now()
            }
        };

        let set_doc = update_doc.get_document_mut("$set").unwrap();

        if let Some(name) = update.name {
            set_doc.insert("name", name);
        }
        if let Some(category) = update.category {
            set_doc.insert("category", category);
        }
        if let Some(brand) = update.brand {
            set_doc.insert("brand", brand);
        }
        if let Some(description) = update.description {
            set_doc.insert("description", description);
        }
        if let Some(purchase_price) = update.purchase_price {
            set_doc.insert("purchase_price", purchase_price);
        }
        if let Some(selling_price) = update.selling_price {
            set_doc.insert("selling_price", selling_price);
        }
        if let Some(mrp) = update.mrp {
            set_doc.insert("mrp", mrp);
        }
        if let Some(gst_rate) = update.gst_rate {
            set_doc.insert("gst_rate", gst_rate);
        }
        if let Some(stock_quantity) = update.stock_quantity {
            set_doc.insert("stock_quantity", stock_quantity);
        }
        if let Some(unit) = update.unit {
            set_doc.insert("unit", unit);
        }
        if let Some(min_stock_level) = update.min_stock_level {
            set_doc.insert("min_stock_level", min_stock_level);
        }
        if let Some(sku) = update.sku {
            set_doc.insert("sku", sku);
        }
        if let Some(barcode) = update.barcode {
            set_doc.insert("barcode", barcode);
        }
        if let Some(has_expiry) = update.has_expiry {
            set_doc.insert("has_expiry", has_expiry);
        }
        if let Some(manufacturing_date) = update.manufacturing_date {
            set_doc.insert("manufacturing_date", manufacturing_date);
        }
        if let Some(expiry_date) = update.expiry_date {
            set_doc.insert("expiry_date", expiry_date);
        }
        if let Some(batch_number) = update.batch_number {
            set_doc.insert("batch_number", batch_number);
        }
        if let Some(supplier_name) = update.supplier_name {
            set_doc.insert("supplier_name", supplier_name);
        }
        if let Some(supplier_contact) = update.supplier_contact {
            set_doc.insert("supplier_contact", supplier_contact);
        }
        if let Some(is_active) = update.is_active {
            set_doc.insert("is_active", is_active);
        }

        // Update and return the updated document
        let product = self
            .collection
            .find_one_and_update(
                doc! { "_id": object_id },
                update_doc,
                mongodb::options::FindOneAndUpdateOptions::builder()
                    .return_document(mongodb::options::ReturnDocument::After)
                    .build(),
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        Ok(product)
    }

    async fn delete(&self, id: &str) -> Result<(), AppError> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest("Invalid product ID".to_string()))?;

        let result = self
            .collection
            .delete_one(doc! { "_id": object_id }, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.deleted_count == 0 {
            return Err(AppError::NotFound("Product not found".to_string()));
        }

        Ok(())
    }

    async fn update_stock(&self, id: &str, quantity_change: f64) -> Result<Product, AppError> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest("Invalid product ID".to_string()))?;

        let product = self
            .collection
            .find_one_and_update(
                doc! { "_id": object_id },
                doc! {
                    "$inc": { "stock_quantity": quantity_change },
                    "$set": { "updated_at": Utc::now() }
                },
                mongodb::options::FindOneAndUpdateOptions::builder()
                    .return_document(mongodb::options::ReturnDocument::After)
                    .build(),
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        Ok(product)
    }

    async fn get_low_stock_products(&self) -> Result<Vec<Product>, AppError> {
        let filter = doc! {
            "$expr": {
                "$lte": ["$stock_quantity", "$min_stock_level"]
            },
            "is_active": true
        };

        let mut cursor = self
            .collection
            .find(filter, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut products = Vec::new();
        while cursor.advance().await.map_err(|e| AppError::DatabaseError(e.to_string()))? {
            let product = cursor
                .deserialize_current()
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            products.push(product);
        }

        Ok(products)
    }

    async fn get_expiring_products(&self, days: i64) -> Result<Vec<Product>, AppError> {
        let now = Utc::now();
        let future_date = now + chrono::Duration::days(days);

        let filter = doc! {
            "has_expiry": true,
            "expiry_date": {
                "$gte": now,
                "$lte": future_date
            },
            "is_active": true
        };

        let mut cursor = self
            .collection
            .find(filter, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut products = Vec::new();
        while cursor.advance().await.map_err(|e| AppError::DatabaseError(e.to_string()))? {
            let product = cursor
                .deserialize_current()
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            products.push(product);
        }

        Ok(products)
    }

    async fn get_expired_products(&self) -> Result<Vec<Product>, AppError> {
        let filter = doc! {
            "has_expiry": true,
            "expiry_date": {
                "$lt": Utc::now()
            },
            "is_active": true
        };

        let mut cursor = self
            .collection
            .find(filter, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut products = Vec::new();
        while cursor.advance().await.map_err(|e| AppError::DatabaseError(e.to_string()))? {
            let product = cursor
                .deserialize_current()
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            products.push(product);
        }

        Ok(products)
    }
}