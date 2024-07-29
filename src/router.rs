use axum::{routing::get, Router};
use tower_request_id::RequestIdLayer;

use crate::{setup, AppState};

async fn authed(_user: crate::core::user::UserExtractor) -> &'static str {
    "authenticated wawa"
}

pub fn make_router(state: AppState) -> Router {
    axum::Router::new()
        .nest("/", crate::routes::router::make_router())
        .with_state(state)
        .layer(setup::trace_layer!())
        .layer(RequestIdLayer)
}
