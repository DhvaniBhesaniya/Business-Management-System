use crate::{
    errors::AppError,
    models::{UpdateUserRequest, User, UserQueryParams, UserRole},
};
use async_trait::async_trait;
use chrono::Utc;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    Collection, Database,
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<User, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, AppError>;
    async fn find_all(&self, params: UserQueryParams) -> Result<(Vec<User>, u64), AppError>;
    async fn update(&self, id: &str, req: &UpdateUserRequest) -> Result<User, AppError>;
    async fn update_role(&self, id: &str, role: UserRole) -> Result<User, AppError>;
    async fn set_active(&self, id: &str, is_active: bool) -> Result<User, AppError>;
    async fn update_password(&self, id: &str, password_hash: &str) -> Result<(), AppError>;
    async fn delete(&self, id: &str) -> Result<(), AppError>;
    async fn count(&self) -> Result<u64, AppError>;
}

#[derive(Clone)]
pub struct MongoUserRepository {
    collection: Collection<User>,
}

impl MongoUserRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("users"),
        }
    }

    pub async fn create_indexes(&self) -> Result<(), AppError> {
        use mongodb::bson::doc;
        use mongodb::IndexModel;

        let email_index = IndexModel::builder()
            .keys(doc! { "email": 1 })
            .options(
                mongodb::options::IndexOptions::builder()
                    .unique(true)
                    .build(),
            )
            .build();

        self.collection
            .create_index(email_index, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        log::info!("User indexes created successfully");
        Ok(())
    }
}

#[async_trait]
impl UserRepository for MongoUserRepository {
    async fn create(&self, mut user: User) -> Result<User, AppError> {
        let result = self.collection.insert_one(&user, None).await.map_err(|e| {
            if e.to_string().contains("duplicate key") {
                AppError::Conflict("User with this email already exists".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        user.id = Some(result.inserted_id.as_object_id().unwrap());
        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = self
            .collection
            .find_one(doc! { "email": email }, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<User>, AppError> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        let user = self
            .collection
            .find_one(doc! { "_id": object_id }, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    async fn count(&self) -> Result<u64, AppError> {
        let count = self
            .collection
            .count_documents(doc! {}, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(count)
    }

    async fn find_all(&self, params: UserQueryParams) -> Result<(Vec<User>, u64), AppError> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20).min(100);
        let skip = (page - 1) * limit;

        let mut filter = doc! {};

        if let Some(search) = &params.search {
            filter.insert(
                "$or",
                mongodb::bson::to_bson(&vec![
                    doc! { "name":  { "$regex": search, "$options": "i" } },
                    doc! { "email": { "$regex": search, "$options": "i" } },
                ])
                .unwrap(),
            );
        }
        if let Some(role) = &params.role {
            filter.insert("role", role.as_str());
        }
        if let Some(is_active) = params.is_active {
            filter.insert("is_active", is_active);
        }

        let total = self
            .collection
            .count_documents(filter.clone(), None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let options = FindOptions::builder()
            .skip(skip)
            .limit(limit as i64)
            .sort(doc! { "created_at": -1 })
            .build();

        let mut cursor = self
            .collection
            .find(filter, options)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut users = Vec::new();
        while cursor
            .advance()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
        {
            users.push(
                cursor
                    .deserialize_current()
                    .map_err(|e| AppError::DatabaseError(e.to_string()))?,
            );
        }

        Ok((users, total))
    }

    async fn update(&self, id: &str, req: &UpdateUserRequest) -> Result<User, AppError> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        let mut set_doc = doc! { "updated_at": Utc::now() };
        if let Some(name) = &req.name {
            set_doc.insert("name", name);
        }
        if let Some(email) = &req.email {
            set_doc.insert("email", email);
        }

        let user = self
            .collection
            .find_one_and_update(
                doc! { "_id": object_id },
                doc! { "$set": set_doc },
                mongodb::options::FindOneAndUpdateOptions::builder()
                    .return_document(mongodb::options::ReturnDocument::After)
                    .build(),
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user)
    }

    async fn update_role(&self, id: &str, role: UserRole) -> Result<User, AppError> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        let user = self
            .collection
            .find_one_and_update(
                doc! { "_id": object_id },
                doc! { "$set": { "role": role.as_str(), "updated_at": Utc::now() } },
                mongodb::options::FindOneAndUpdateOptions::builder()
                    .return_document(mongodb::options::ReturnDocument::After)
                    .build(),
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user)
    }

    async fn set_active(&self, id: &str, is_active: bool) -> Result<User, AppError> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        let user = self
            .collection
            .find_one_and_update(
                doc! { "_id": object_id },
                doc! { "$set": { "is_active": is_active, "updated_at": Utc::now() } },
                mongodb::options::FindOneAndUpdateOptions::builder()
                    .return_document(mongodb::options::ReturnDocument::After)
                    .build(),
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user)
    }

    async fn update_password(&self, id: &str, password_hash: &str) -> Result<(), AppError> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        let result = self
            .collection
            .update_one(
                doc! { "_id": object_id },
                doc! { "$set": { "password_hash": password_hash, "updated_at": Utc::now() } },
                None,
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.matched_count == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), AppError> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

        let result = self
            .collection
            .delete_one(doc! { "_id": object_id }, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.deleted_count == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }
}
