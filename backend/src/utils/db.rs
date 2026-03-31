use crate::configuration::AppConfig;
use mongodb::{Client, Database};

pub async fn connect(config: &AppConfig) -> Result<Database, mongodb::error::Error> {
    let client = Client::with_uri_str(&config.mongodb_uri).await?;

    // Ping the database to verify connection
    client
        .database("admin")
        .run_command(mongodb::bson::doc! {"ping": 1}, None)
        .await?;

    log::info!("Connected to MongoDB successfully");

    Ok(client.database(&config.database_name))
}
