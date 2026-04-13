use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
            created_at: now,
            updated_at: now,
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

// DTOs (Data Transfer Objects)

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
            created_at: user.created_at,
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
