use crate::AppState;
use crate::db::models;
use crate::models::user;
use anyhow::{Context, Result};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    response::Response,
};
use mongodb::Collection;

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<models::Payload>,
) -> Result<Response, (StatusCode, String)> {
    let db = state.db.clone();
    let payload = payload;

    let user_collection: Collection<user::User> = db.collection("whatsappUser");

    let user = user::User {
        name: payload.name,
        phone_number: payload.phone_number,
        country: payload.country,
        description: None,
    };
    let _insert_result = user_collection
        .insert_one(user)
        .await
        .context("Error inserting user");

    Ok((
        StatusCode::CREATED,
        format!("Succesfully created user in Database"),
    )
        .into_response())
}
