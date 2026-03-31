//! Middleware modules.
//!
//! - [`auth`]         – JWT authentication (`AuthMiddleware`, `extract_claims`)
//! - [`rate_limiter`] – per-IP sliding-window rate limiting (`RateLimiter`)

pub mod auth;
pub mod rate_limiter;

pub use auth::AuthMiddleware;
pub use rate_limiter::RateLimiter;
