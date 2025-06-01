use serde::Deserialize;

#[derive(Deserialize)]
pub struct Payload {
    pub name: Option<String>,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    pub country: String,
}
