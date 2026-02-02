use axum::{extract::Path, Json};
use serde_json::{json, Value};

use crate::server::models::User;
use crate::server::service::UserService;

pub async fn get_user_handler(Path(id): Path<u64>) -> Result<Json<User>, axum::http::StatusCode> {
    match UserService::get_user(id).await {
        Some(user) => Ok(Json(user)),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

pub async fn hello_world() -> Result<Json<Value>, axum::http::StatusCode> {
    let response = json!({
        "status": "ok",
        "message": "Hello World"
    });
    Ok(Json(response))
}
