use axum::response::Html;
use axum::{Json, Router};
use axum::routing::{get};

pub async fn create_app() -> Router {
    Router::new()
        .merge(crate::routes::user::create_route())
        .merge(crate::routes::paintings::create_route())
        .merge(crate::routes::painting::create_route())
        .merge(crate::routes::messages::create_route())
        .route("/hello", get(hello))
        .route("/hello_json", get(hello_json))
}

async fn hello() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn hello_json() -> Json<&'static str> {
    Json("Hello, World!")
}