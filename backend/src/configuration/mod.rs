use config::Config;
mod setting;
use lazy_static::lazy_static;
// use crate::error::{ConfigError, Result};
use serde::de::Deserialize;
use std::{env, sync::RwLock};


#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub mongodb_uri: String,
    pub database_name: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub admin_email: String,
    pub admin_password: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        // Load .env file
        dotenvy::dotenv().ok();

        Ok(AppConfig {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .map_err(|_| "Invalid PORT")?,
            mongodb_uri: env::var("MONGODB_URI")
                .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            database_name: env::var("DATABASE_NAME")
                .unwrap_or_else(|_| "shree_nandi_db".to_string()),
            jwt_secret: env::var("JWT_SECRET").map_err(|_| "JWT_SECRET must be set")?,
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .map_err(|_| "Invalid JWT_EXPIRATION_HOURS")?,
            admin_email: env::var("ADMIN_EMAIL")
                .unwrap_or_else(|_| "admin@shreenandi.com".to_string()),
            admin_password: env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "Admin@123".to_string()),
        })
    }

    pub fn server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}


lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(setting::get_config());
}

// pub fn get(property:String) -> String{
//     CONFIG.lock().unwrap().get(&property).unwrap()
// }

pub fn get<'de, T: Deserialize<'de>>(key: &str) -> T {
    let res = CONFIG.read().unwrap().get(key);
    match res {
        Ok(val) => val,
        Err(e) => {
            println!("Failed to get config key {}: {}", key, e);
            panic!("Configuration key not found");
        }
    }
}

pub fn get_res<'de, T: Deserialize<'de>>(key: &str) -> Result<T, config::ConfigError> {
    CONFIG.read().unwrap().get(key)
}
