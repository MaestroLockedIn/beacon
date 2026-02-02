use axum::{routing::get, Router};

use crate::server::handler;

pub fn configure_routes() -> Router {
    Router::new()
        .route("/api", get(handler::hello_world))
        .route("/api/users/{:id}", get(handler::get_user_handler))
}
