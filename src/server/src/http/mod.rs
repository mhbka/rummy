use crate::config::Config;
use anyhow::Context;
use axum::{http::header::AUTHORIZATION, Router};
use sqlx::PgPool;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};
use tokio::net::TcpListener;

// Utility modules.

/// Defines a common error type to use for all request handlers, compliant with the Realworld spec.
mod error;

/// Contains definitions for application-specific parameters to handler functions,
/// such as `AuthUser` which checks for the `Authorization: Token <token>` header in the request,
/// verifies `<token>` as a JWT and checks the signature,
/// then deserializes the information it contains.
// mod extractor;

/// A catch-all module for other common types in the API. Arguably, the `error` and `extractor`
/// modules could have been children of this one, but that's more of a subjective decision.
// mod types;

// Modules introducing API routes. The names match the routes listed in the Realworld spec,
// although the `articles` module also includes the `GET /api/tags` route because it touches
// the `article` table.
//
// This is not the order they were written in; `rustfmt` auto-sorts them.
// However, you should follow the order they were written in because some of the comments
// are more stream-of-consciousness and assume you read them in a particular order.
//
// See `api_router()` below for the recommended order.
// mod articles;
// mod profiles;
mod users;

pub use error::{HttpError, ResultExt};

use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer,
    sensitive_headers::SetSensitiveHeadersLayer, timeout::TimeoutLayer, trace::TraceLayer,
};

/// The core type through which handler functions can access common API state.
///
/// This can be accessed by adding a parameter `State<ApiContext>` to a handler function's
/// parameters.
///
/// In other projects I've passed this stuff as separate objects, e.g.
/// using a separate actix-web `Data` extractor for each of `Config`, `PgPool`, etc.
/// It just ends up being kind of annoying that way, but does have the whole
/// "pass only what you need where you need it" angle.
///
/// It may not be a bad idea if you need your API to be more modular (turn routes
/// on and off, and disable any unused extension objects) but it's really up to a
/// judgement call.
#[derive(Clone)]
pub(crate) struct AppState {
    config: Arc<Config>,
    db: PgPool,
}


/// Sets up and starts the server.
pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let app_state = AppState {
        config: Arc::new(config),
        db,
    };

    let app = api_router(app_state);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("error running HTTP server")
}


/// Creates the main API router and combines all other routers.
fn api_router(state: AppState) -> Router {
    // TODO: add other routers as merge() calls here
    Router::new()
        .merge(users::router())
    
        // Enables logging. Use `RUST_LOG=tower_http=debug`
        .layer((
            SetSensitiveHeadersLayer::new([AUTHORIZATION]),
            CompressionLayer::new(),
            TraceLayer::new_for_http().on_failure(()),
            TimeoutLayer::new(Duration::from_secs(30)),
            CatchPanicLayer::new(),
        ))
        .with_state(state)
}


/// Sends a shutdown signal to the server if ctrl+c is pressed.
async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
