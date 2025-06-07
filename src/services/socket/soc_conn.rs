use std::sync::Arc;

use crate::AppState;
use crate::services::socket::models;
use anyhow::Result;
use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
};
use axum::{response::IntoResponse, response::Response};
use dashmap::DashMap;
use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use serde_json;
use tokio::sync::{broadcast, mpsc};

use tracing::{error, info};
type Users = Arc<DashMap<String, mpsc::UnboundedSender<Message>>>;
type Groups = Arc<DashMap<String, Vec<String>>>;
type Groups_tx = Arc<DashMap<String, Arc<broadcast::Sender<String>>>>;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Result<Response, (StatusCode, String)> {
    info!("websocket connection started");
    let state = state.clone();
    let users = state.user_map.clone();
    let groups = state.group_map.clone();
    let groups_tx = state.group_tx.clone();

    Ok(ws
        .on_upgrade(move |socket| handle_socket(socket, users, groups, groups_tx))
        .into_response())
}

/**
 * whenever user connect give him tx for his userId
 */
async fn handle_socket(mut socket: WebSocket, users: Users, groups: Groups, group_tx: Groups_tx) {
    let Some(Ok(Message::Text(user_id))) = socket.next().await else {
        return;
    };
    //create the unbounded channel
    let (tx, mut rx) = mpsc::unbounded_channel();
    {
        info!("websocket created for userId: {}", user_id);
        users.insert(user_id.to_string(), tx);
    }

    let (mut sender, mut receiver) = socket.split();

    let users_clone = users.clone();
    let user_id_clone = user_id.clone();

    while let Some(Ok(Message::Text(text))) = receiver.next().await {
        if let Ok(chat_msg) = serde_json::from_str::<models::ChatMessage>(&text) {
            info!("Chat message is {:#?}", chat_msg);

            if chat_msg.is_group {
                
            }

            if let Some(tx) = users.get(&chat_msg.to) {
                let msg_txt = serde_json::to_string(&chat_msg).unwrap();
                info!("Message sent to {}", chat_msg.to);
                if let Err(e) = tx.send(Message::Text(msg_txt.into())) {
                    error!(
                        "Error sending message to userID {} with error {:#?}",
                        chat_msg.to, e
                    );
                }
            } else {
                error!("Error sending to user {}", chat_msg.to);
            }
        }
    }
}

//    //take a look at rx for the user to receive dedicated message to him
//    tokio::spawn(async move {
//     while let Some(msg) = rx.recv().await {
//         if sender.send(msg).await.is_err() {
//             info!("User {} disconnected", user_id_clone);
//             users_clone.remove(&user_id_clone.to_string());
//             break;
//         }
//     }
// });
