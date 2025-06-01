use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    from: String,
    to: String,
    content: String,
}
