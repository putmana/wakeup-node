use crate::api::app;
use anyhow::Result;

mod api;
mod config;
mod core;

pub async fn start() -> Result<()> {
    app::start().await?;

    Ok(())
}
