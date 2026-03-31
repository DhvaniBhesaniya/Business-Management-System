//! JWT-based authentication middleware.
//!
//! Usage in router:
//! ```rust
//! .layer(middleware::from_fn_with_state(jwt_utils.clone(), AuthMiddleware::auth))
//! ```
//! Handlers then retrieve the verified claims via:
//! ```rust
//! Extension(claims): Extension<Claims>
//! ```

use std::sync::Arc;
use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use crate::{errors::AppError, utils::jwt::JwtUtils};

// Claims are injected into request extensions by AuthMiddleware::auth.
// Handlers retrieve them with the axum extractor: Extension(claims): Extension<Claims>

pub struct AuthMiddleware;

impl AuthMiddleware {
    /// Validates the `Authorization: Bearer <token>` header.
    /// On success the decoded [`Claims`] are stored in request extensions so
    /// downstream handlers can retrieve them with `Extension<Claims>`.
    pub async fn auth(
        State(jwt_utils): State<Arc<JwtUtils>>,
        mut req: Request,
        next: Next,
    ) -> Result<Response, AppError> {
        let token = extract_token_from_header(&req)?;
        let claims = jwt_utils.verify_token(&token)?;
        req.extensions_mut().insert(claims);
        Ok(next.run(req).await)
    }
}

/// Extracts the raw token string from `Authorization: Bearer <token>`.
fn extract_token_from_header(req: &Request) -> Result<String, AppError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or_else(|| AppError::AuthError("Missing authorization header".to_string()))?
        .to_str()
        .map_err(|_| AppError::AuthError("Invalid authorization header encoding".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::AuthError(
            "Invalid authorization format – expected: Bearer <token>".to_string(),
        ));
    }

    Ok(auth_header[7..].to_string())
}
