use anyhow::{Context, Result};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::AppState;
use crate::services::db::models::Group;
use mongodb::Collection;

/**
 * creating group
 */
pub async fn create_group(
    State(state): State<AppState>,
    Json(payload): Json<Group>,
) -> Result<Response, (StatusCode, String)> {
    if payload.members.len() < 3 {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Add atleast 3 users to create the group"),
        ));
    };

    let db = state.db.clone();
    let group_collection: mongodb::Collection<Group> = db.collection("groups");

    let _ = group_collection
        .insert_one(payload)
        .await
        .context("Error inserting group");

    Ok((StatusCode::CREATED, format!("Group created succesfully")).into_response())
}

/**
 * add more users to group
 */

pub async fn add_users_to_group(
    State(state): State<AppState>,
    Json(payload): Json<Vec<String>>,
) -> Result<Response, (StatusCode, String)> {
    let db = state.db.clone();
    let collection: Collection<Group> = db.collection("groups");
    let _ = collection.update_one(doc!{
        name:payload.name;
    }, update)
    Ok((StatusCode::OK, format!("User Added Succesfully")).into_response())
}
