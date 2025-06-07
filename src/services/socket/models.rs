use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub from: String,
    pub to: String,
    pub msg: String,
    pub is_group: bool,
}
