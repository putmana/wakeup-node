use axum::{Router, routing::get};

use crate::api::app::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(ping))
}

pub async fn ping() -> &'static str {
    "Pong"
}
