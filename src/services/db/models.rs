use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Payload {
    pub name: Option<String>,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    pub country: String,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub members: Vec<String>,
}
