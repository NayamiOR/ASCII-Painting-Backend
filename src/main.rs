mod routes;
mod app;

use std::env;
use axum::{response::{Html, Json}, routing::get, Router};
use axum_macros::debug_handler;

#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    let url = "localhost:3000";
    let app = app::create_app().await;

    let listener = tokio::net::TcpListener::bind(url).await.unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();
}

