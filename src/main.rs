use anyhow::{Context, Result};
use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    response::Response,
    routing::{get, post},
};
use dotenv::dotenv;
use mongodb::Database;
use services::db::create_user::create_user;
mod models;
mod services;
use crate::services::db;
use std::{collections::HashMap, sync::Arc};
use tokio;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub user_map: Arc<RwLock<HashMap<String, String>>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let db = Arc::new(db::connect_db::connect_db().await?);
    let user_map = Arc::new(RwLock::new(HashMap::new()));
    let appstate = AppState {
        db: db,
        user_map: user_map,
    };

    let app = Router::new()
        .route("/", get(|| async { format!("Hello its working") }))
        .route("/create_user", post(create_user))
        .with_state(appstate);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
