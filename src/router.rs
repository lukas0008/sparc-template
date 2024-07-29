use axum::Router;
use tower_request_id::RequestIdLayer;

use crate::{setup, STATE};

pub fn make_router() -> Router {
    axum::Router::new()
        .nest("/", crate::routes::router::make_router())
        .with_state(STATE.clone())
        .layer(setup::trace_layer!())
        .layer(RequestIdLayer)
}
