use std::sync::Arc;
use validator::Validate;
use crate::{
    errors::AppError,
    models::{
        ChangePasswordRequest, CreateUserRequest, LoginRequest, LoginResponse,
        RegisterRequest, SetUserActiveRequest, UpdateUserRequest, UpdateUserRoleRequest,
        User, UserListResponse, UserQueryParams, UserResponse, UserRole,
    },
    repositories::UserRepository,
    utils::jwt::{hash_password, verify_password, JwtUtils},
};

#[derive(Clone)]
pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    jwt_utils: Arc<JwtUtils>,
}

impl AuthService {
    pub fn new(user_repo: Arc<dyn UserRepository>, jwt_utils: Arc<JwtUtils>) -> Self {
        Self {
            user_repo,
            jwt_utils,
        }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<LoginResponse, AppError> {
        // Validate input
        req.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        // Check if user already exists
        if let Some(_) = self.user_repo.find_by_email(&req.email).await? {
            return Err(AppError::Conflict(
                "User with this email already exists".to_string(),
            ));
        }

        // Hash password
        let password_hash = hash_password(&req.password)?;

        // Create user (first user is admin, others need to be invited by admin later)
        let user_count = self.user_repo.count().await?;
        let role = if user_count == 0 {
            UserRole::Admin
        } else {
            // For now, we'll only allow admin registration
            // Later, admins can invite other users
            return Err(AppError::Forbidden(
                "Registration is closed. Contact administrator.".to_string(),
            ));
        };

        let user = User::new(req.email, password_hash, req.name, role);

        let created_user = self.user_repo.create(user).await?;

        // Generate token
        let token = self.jwt_utils.generate_token(&created_user)?;

        Ok(LoginResponse {
            token,
            user: UserResponse::from(created_user),
        })
    }

    pub async fn login(&self, req: LoginRequest) -> Result<LoginResponse, AppError> {
        // Validate input
        req.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        // Find user by email
        let user = self
            .user_repo
            .find_by_email(&req.email)
            .await?
            .ok_or_else(|| AppError::AuthError("Invalid email or password".to_string()))?;

        // Check if user is active
        if !user.is_active {
            return Err(AppError::Forbidden("Account is deactivated".to_string()));
        }

        // Verify password
        let is_valid = verify_password(&req.password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::AuthError("Invalid email or password".to_string()));
        }

        // Generate token
        let token = self.jwt_utils.generate_token(&user)?;

        Ok(LoginResponse {
            token,
            user: UserResponse::from(user),
        })
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<UserResponse, AppError> {
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(UserResponse::from(user))
    }

    // ── Admin: create a user with an explicit role ───────────────────────────

    pub async fn create_user_by_admin(
        &self,
        req: CreateUserRequest,
        caller_role: &UserRole,
    ) -> Result<UserResponse, AppError> {
        if *caller_role != UserRole::Admin {
            return Err(AppError::Forbidden(
                "Only admins can create users".to_string(),
            ));
        }

        req.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        if self.user_repo.find_by_email(&req.email).await?.is_some() {
            return Err(AppError::Conflict(
                "User with this email already exists".to_string(),
            ));
        }

        let password_hash = hash_password(&req.password)?;
        let user = User::new(req.email, password_hash, req.name, req.role);
        let created = self.user_repo.create(user).await?;
        Ok(UserResponse::from(created))
    }

    // ── Admin: list all users with pagination / filters ──────────────────────

    pub async fn list_users(
        &self,
        params: UserQueryParams,
        caller_role: &UserRole,
    ) -> Result<UserListResponse, AppError> {
        if *caller_role != UserRole::Admin && *caller_role != UserRole::Manager {
            return Err(AppError::Forbidden(
                "Insufficient permissions to list users".to_string(),
            ));
        }

        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20).min(100);
        let (users, total) = self.user_repo.find_all(params).await?;
        let total_pages = (total as f64 / limit as f64).ceil() as u64;

        Ok(UserListResponse {
            users: users.into_iter().map(UserResponse::from).collect(),
            total,
            page,
            limit,
            total_pages,
        })
    }

    // ── Update name / email (admin or self) ──────────────────────────────────

    pub async fn update_user(
        &self,
        user_id: &str,
        req: UpdateUserRequest,
        caller_id: &str,
        caller_role: &UserRole,
    ) -> Result<UserResponse, AppError> {
        // Only the user themselves or an admin may update the profile.
        if caller_id != user_id && *caller_role != UserRole::Admin {
            return Err(AppError::Forbidden(
                "You can only update your own profile".to_string(),
            ));
        }

        req.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        // Guard against email collision when trying to change email.
        if let Some(new_email) = &req.email {
            if let Some(existing) = self.user_repo.find_by_email(new_email).await? {
                if existing.id.as_ref().unwrap().to_hex() != user_id {
                    return Err(AppError::Conflict(
                        "Email is already in use".to_string(),
                    ));
                }
            }
        }

        let updated = self.user_repo.update(user_id, &req).await?;
        Ok(UserResponse::from(updated))
    }

    // ── Change own password ───────────────────────────────────────────────────

    pub async fn change_password(
        &self,
        user_id: &str,
        req: ChangePasswordRequest,
    ) -> Result<(), AppError> {
        req.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        if !verify_password(&req.current_password, &user.password_hash)? {
            return Err(AppError::AuthError("Current password is incorrect".to_string()));
        }

        let new_hash = hash_password(&req.new_password)?;
        self.user_repo.update_password(user_id, &new_hash).await?;
        Ok(())
    }

    // ── Admin: change a user's role ───────────────────────────────────────────

    pub async fn update_user_role(
        &self,
        user_id: &str,
        req: UpdateUserRoleRequest,
        caller_role: &UserRole,
    ) -> Result<UserResponse, AppError> {
        if *caller_role != UserRole::Admin {
            return Err(AppError::Forbidden(
                "Only admins can change user roles".to_string(),
            ));
        }

        let updated = self.user_repo.update_role(user_id, req.role).await?;
        Ok(UserResponse::from(updated))
    }

    // ── Admin: activate or deactivate a user ─────────────────────────────────

    pub async fn set_user_active(
        &self,
        user_id: &str,
        req: SetUserActiveRequest,
        caller_role: &UserRole,
    ) -> Result<UserResponse, AppError> {
        if *caller_role != UserRole::Admin {
            return Err(AppError::Forbidden(
                "Only admins can activate or deactivate accounts".to_string(),
            ));
        }

        let updated = self.user_repo.set_active(user_id, req.is_active).await?;
        Ok(UserResponse::from(updated))
    }

    // ── Admin: hard-delete a user ─────────────────────────────────────────────

    pub async fn delete_user(
        &self,
        user_id: &str,
        caller_id: &str,
        caller_role: &UserRole,
    ) -> Result<(), AppError> {
        if *caller_role != UserRole::Admin {
            return Err(AppError::Forbidden(
                "Only admins can delete users".to_string(),
            ));
        }
        if caller_id == user_id {
            return Err(AppError::BadRequest(
                "You cannot delete your own account".to_string(),
            ));
        }

        self.user_repo.delete(user_id).await
    }
}