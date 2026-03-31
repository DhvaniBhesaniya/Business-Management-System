//! Per-IP sliding-window rate-limiter middleware.
//!
//! Each unique IP address is allowed at most `max_requests` requests within
//! a rolling `window_secs`-second window.  Requests that exceed the limit
//! receive a `429 Too Many Requests` response.
//!
//! Usage in router:
//! ```rust
//! let limiter = RateLimiter::new(100, 60); // 100 req / 60 s
//! Router::new()
//!     …
//!     .layer(middleware::from_fn_with_state(limiter, RateLimiter::rate_limit))
//! ```
//! The router **must** be served with `into_make_service_with_connect_info::<SocketAddr>()`
//! so that the peer address is available to this middleware.

use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use axum::{
    extract::{ConnectInfo, Request, State},
    middleware::Next,
    response::Response,
};
use crate::errors::AppError;

/// Shared, cheaply-cloneable rate-limiter state.
#[derive(Clone)]
pub struct RateLimiter {
    /// Map of IP → timestamps of requests within the current window.
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    /// Maximum number of requests permitted inside the window.
    max_requests: usize,
    /// Length of the sliding window.
    window_duration: Duration,
}

impl RateLimiter {
    /// Create a new `RateLimiter`.
    ///
    /// * `max_requests` – ceiling on requests per IP per window.
    /// * `window_secs`  – length of the sliding window in seconds.
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_duration: Duration::from_secs(window_secs),
        }
    }

    /// Axum middleware function.
    ///
    /// Checks the caller's IP against the sliding window; returns
    /// `429 Too Many Requests` if the limit is exceeded, otherwise forwards
    /// the request to the next handler.
    pub async fn rate_limit(
        State(limiter): State<RateLimiter>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        req: Request,
        next: Next,
    ) -> Result<Response, AppError> {
        let ip = addr.ip().to_string();
        let now = Instant::now();

        {
            let mut map = limiter
                .requests
                .lock()
                .map_err(|_| AppError::InternalError("Rate limiter lock poisoned".to_string()))?;

            let timestamps = map.entry(ip).or_default();

            // Remove timestamps that have fallen outside the current window.
            timestamps.retain(|&t| now.duration_since(t) < limiter.window_duration);

            if timestamps.len() >= limiter.max_requests {
                return Err(AppError::RateLimitExceeded(
                    "Too many requests – please slow down.".to_string(),
                ));
            }

            timestamps.push(now);
        }

        Ok(next.run(req).await)
    }
}
