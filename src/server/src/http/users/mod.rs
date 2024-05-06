use axum::Router;
use super::AppState;

pub mod auth;
pub mod types;
pub mod routes;
pub mod handlers;
pub mod util;

/// Nest all the routes into this 1 router.
pub(super) fn router() -> Router<AppState> {
    Router::new()
        .nest("/u", routes::router())
}