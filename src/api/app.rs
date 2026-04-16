use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use tokio::sync::Mutex;

use crate::{api::v1, config::config::Config, core::player::AlarmPlayer};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub player: Arc<Mutex<AlarmPlayer>>,
}

pub async fn start() -> Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    let state = AppState {
        config: Arc::new(config),
        player: Arc::new(Mutex::new(AlarmPlayer::new())),
    };

    let app = Router::new()
        .nest("/api/v1", v1::router())
        .with_state(state);

    let bind_host = "0.0.0.0:8000";

    let listener = tokio::net::TcpListener::bind(bind_host).await?;
    println!("Listening on {bind_host}");

    axum::serve(listener, app).await?;

    Ok(())
}
