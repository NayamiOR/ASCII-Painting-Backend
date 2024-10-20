use crate::ApiContext;
use axum::http::Request;
use axum::response::Html;
use axum::routing::get;
use axum::{http, Json, Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub async fn api_router() -> Router {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let config: std::collections::HashMap<String, String> = std::env::vars().collect();

    let cors = CorsLayer::new()
        .allow_origin(http::HeaderValue::from_static("http://127.0.0.1:5173"))
        .allow_methods(vec![
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
        ])
        .allow_headers(vec![
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
        ])
        .allow_credentials(true);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(100)
        .connect(&database_url)
        .await
        .expect("数据库连接失败");

    let api_context = ApiContext { config, pool };

    Router::new()
        .merge(crate::routes::user::create_route())
        .merge(crate::routes::paintings::create_route())
        .merge(crate::routes::painting::create_route())
        .merge(crate::routes::messages::create_route())
        .with_state(api_context)
        .route("/hello", get(hello))
        .route("/hello_json", get(hello_json))
        .layer(cors)
        .layer(TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            tracing::info_span!(
                "request",
                method = %request.method(),
                uri = %request.uri(),
                version = ?request.version(),
            )
        }))
}

async fn hello() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn hello_json() -> Json<&'static str> {
    Json("Hello, World!")
}
