use axum::Router;

use crate::api::app::AppState;

mod alarm;
mod ping;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/ping", ping::router())
        .nest("/alarm", alarm::router())
}
