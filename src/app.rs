use crate::ApiContext;
use axum::response::Html;
use axum::routing::get;
use axum::{Json, Router};

pub async fn api_router() -> Router {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let config: std::collections::HashMap<String, String> = std::env::vars().collect();

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
}

async fn hello() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn hello_json() -> Json<&'static str> {
    Json("Hello, World!")
}
