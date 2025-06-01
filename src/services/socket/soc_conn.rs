use std::{collections::HashMap, sync::Arc};

use crate::AppState;
use anyhow::Result;
use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
};
use axum::{response::IntoResponse, response::Response};
use futures_util::stream::StreamExt;
use tokio::sync::{RwLock, mpsc};

type Users = Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>>;
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Result<Response, (StatusCode, String)> {
    let user = state.clone().user_map;
    ws.on_upgrade(move |socket| handle_socket(socket, user.clone()));
    Ok((StatusCode::CONTINUE, format!("Connection done")).into_response())
}

async fn handle_socket(mut socket: WebSocket, users: Arc<RwLock<HashMap<String, String>>>) {
    let Some(Ok(Message::Text(user_id))) = socket.next().await else {
        return;
    };
    println!("User connected: {}", user_id);
}
