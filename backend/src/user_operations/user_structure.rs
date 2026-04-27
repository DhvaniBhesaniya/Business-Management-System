use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    Collection, Database,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::errors::AppError;

// ── User Role ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Manager,
    Cashier,
}

impl UserRole {
    pub fn as_str(&self) -> &str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Manager => "manager",
            UserRole::Cashier => "cashier",
        }
    }
}

// ── User Entity ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    // #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: bson::DateTime,
    pub updated_at: bson::DateTime,
}

impl User {
    pub fn new(email: String, password_hash: String, name: String, role: UserRole) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            email,
            password_hash,
            name,
            role,
            is_active: true,
            created_at: now.into(),
            updated_at: now.into(),
        }
    }

    pub fn has_permission(&self, required_role: &UserRole) -> bool {
        match (&self.role, required_role) {
            (UserRole::Admin, _) => true, // Admin has all permissions
            (UserRole::Manager, UserRole::Manager) => true,
            (UserRole::Manager, UserRole::Cashier) => true,
            (UserRole::Cashier, UserRole::Cashier) => true,
            _ => false,
        }
    }
}

// ── DTOs (Data Transfer Objects) ─────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    
    #[validate(length(min = 2, message = "Name must be at least 2 characters"))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.unwrap().to_hex(),
            email: user.email,
            name: user.name,
            role: user.role,
            is_active: user.is_active,
            created_at: user.created_at.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub email: String,
    pub role: UserRole,
    pub exp: i64, // expiration timestamp
    pub iat: i64, // issued at timestamp
}

// ── Admin-facing user management DTOs ───────────────────────────────────────

/// Admin creates a new user with an explicit role.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(length(min = 2, message = "Name must be at least 2 characters"))]
    pub name: String,

    pub role: UserRole,
}

/// Update a user's mutable profile fields (name and/or email).
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: Option<String>,

    #[validate(length(min = 2, message = "Name must be at least 2 characters"))]
    pub name: Option<String>,
}

/// Change the authenticated user's own password.
#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1, message = "Current password is required"))]
    pub current_password: String,

    #[validate(length(min = 8, message = "New password must be at least 8 characters"))]
    pub new_password: String,
}

/// Admin changes a user's role.
#[derive(Debug, Deserialize)]
pub struct UpdateUserRoleRequest {
    pub role: UserRole,
}

/// Admin activates or deactivates a user account.
#[derive(Debug, Deserialize)]
pub struct SetUserActiveRequest {
    pub is_active: bool,
}

/// Query parameters for GET /api/users.
#[derive(Debug, Deserialize)]
pub struct UserQueryParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub search: Option<String>,
    pub role: Option<UserRole>,
    pub is_active: Option<bool>,
}

/// Paginated list of users returned to the caller.
#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
    pub total_pages: u64,
}

// ── Repository Trait ─────────────────────────────────────────────────────────

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

// ── MongoDB Repository Implementation ───────────────────────────────────────

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
