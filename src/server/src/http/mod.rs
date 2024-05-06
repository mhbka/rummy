use crate::config::Config;
use anyhow::Context;
use tokio::net::TcpListener;
use axum::{http::header::AUTHORIZATION, Router};
use sqlx::PgPool;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};
use tower_http::{
    catch_panic::CatchPanicLayer, 
    compression::CompressionLayer,
    sensitive_headers::SetSensitiveHeadersLayer, 
    timeout::TimeoutLayer, 
    trace::TraceLayer
};

/// Common error type that maps to HTTP errors, which can be returned as Responses
mod error;
pub use error::{HttpError, ResultExt};

/// A catch-all module for other common types in the API. Arguably, the `error` and `extractor`
/// modules could have been children of this one, but that's more of a subjective decision.
// mod types;

/// API routes
mod users;

/// The core state of the app.
/// 
/// **Note**: Substates are an option if it is better/more performant to hold smaller pieces of state.
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
fn api_router(app_state: AppState) -> Router {
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
        .with_state(app_state)
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
