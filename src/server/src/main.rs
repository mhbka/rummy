use axum::{
    routing::get,
    Router,
};

use clap::Parser;
use server::config::Config;
use server::http;

#[tokio::main]
async fn main() {
    // Load .env file if it exists
    dotenv::dotenv().ok();

    // Load config
    let config = Config::parse();

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

