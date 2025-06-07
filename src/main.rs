use anyhow::{Context, Result};
use axum::{
    Router,
    extract::{Json, State, ws::Message},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use dotenv::dotenv;
use mongodb::Database;
use services::db::create_group::create_group;
use services::db::create_user::create_user;
mod models;
mod services;
use crate::services::db::{self, create_group};
use crate::services::socket::soc_conn::ws_handler;
use dashmap::DashMap;
use std::sync::Arc;
use tokio;
use tokio::sync::mpsc;

//broadcasting
use tokio::sync::broadcast;
//Logging
use tracing::info;
use tracing_appender::rolling;
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub user_map: Arc<DashMap<String, mpsc::UnboundedSender<Message>>>,
    pub group_map: Arc<DashMap<String, Vec<String>>>,
    pub group_tx: Arc<DashMap<String, Arc<broadcast::Sender<String>>>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let file_appender = rolling::daily("logs", "myapp.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter("info")
        .init();

    info!("Logging initialised");
    dotenv().ok();
    let db = Arc::new(db::connect_db::connect_db().await?);
    let user_map = Arc::new(DashMap::new());
    let group_map = Arc::new(DashMap::new());
    let group_tx = Arc::new(DashMap::new());
    let appstate = AppState {
        db,
        user_map,
        group_map,
        group_tx,
    };

    let app = Router::new()
        .route("/", get(|| async { format!("Hello its working") }))
        .route("/create_user", post(create_user))
        .route("/send_message", get(ws_handler))
        .route("/create_group", post(create_group))
        .with_state(appstate);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
