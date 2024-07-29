use crate::AppState;
use axum::{routing::get, Router};

use super::{authed, index};

pub fn make_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index::index_route))
        .route("/auth", get(authed::authed_route))
}
