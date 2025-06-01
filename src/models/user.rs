use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub name: Option<String>,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    pub country: String,
    pub description: Option<String>,
}
