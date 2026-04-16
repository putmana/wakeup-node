use axum::{Json, Router, extract::State, routing::post};
use serde::Serialize;

use crate::api::{app::AppState, error::ApiError};

#[derive(Serialize)]
pub struct StatusResponse {
    active: bool,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/start", post(start_alarm))
        .route("/stop", post(stop_alarm))
}

async fn start_alarm(State(state): State<AppState>) -> Result<Json<StatusResponse>, ApiError> {
    let mut player = state.player.lock().await;
    player
        .start(
            &state.config.alarm_file,
            &state.config.audio_backend,
            &state.config.audio_device,
        )
        .await?;

    Ok(Json(StatusResponse { active: true }))
}

async fn stop_alarm(State(state): State<AppState>) -> Result<Json<StatusResponse>, ApiError> {
    let mut player = state.player.lock().await;
    player.stop().await?;

    Ok(Json(StatusResponse { active: false }))
}
