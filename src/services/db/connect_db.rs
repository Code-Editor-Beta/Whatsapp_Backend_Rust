use anyhow::{Context, Result};
use mongodb::{Client, Database};
use std::env;

/**
 * function to connect to db
 */
pub async fn connect_db() -> Result<Database> {
    let uri = env::var("MONGODB_URI").context("You must set MONGODB_URI environment var!")?;
    let client = Client::with_uri_str(&uri)
        .await
        .context("Failed to connect to MongoDb")?;
    let db = client.database("backend-Rust");
    Ok(db)
}
