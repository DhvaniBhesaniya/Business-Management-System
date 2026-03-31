use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::{configuration::AppConfig, errors::AppError, models::{Claims, User}};

pub struct JwtUtils {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiration_hours: i64,
}

impl JwtUtils {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(config.jwt_secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            expiration_hours: config.jwt_expiration_hours,
        }
    }

    pub fn generate_token(&self, user: &User) -> Result<String, AppError> {
        let now = Utc::now();
        let exp = (now + chrono::Duration::hours(self.expiration_hours)).timestamp();
        
        let claims = Claims {
            sub: user.id.as_ref().unwrap().to_hex(),
            email: user.email.clone(),
            role: user.role.clone(),
            exp,
            iat: now.timestamp(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::InternalError(format!("Failed to generate token: {}", e)))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| AppError::AuthError(format!("Invalid token: {}", e)))
    }
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::InternalError(format!("Failed to hash password: {}", e)))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash)
        .map_err(|e| AppError::InternalError(format!("Failed to verify password: {}", e)))
}