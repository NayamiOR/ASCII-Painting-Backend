#![allow(unused)]

mod app;
mod app_config;
mod config;
mod dao;
mod error;
mod log;
mod models;
mod routes;
mod utils;

use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::env;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // check vars
    let _ = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let _ = env::var("DATA_DIR").expect("DATA_DIR is not set");
    // let _ = env::var("JWT_SECRET").expect("JWT_SECRET is not set");

    let config: HashMap<String, String> = env::vars().collect();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let url = format!(
        "localhost:{}",
        config.get("PORT").unwrap_or(&"3000".to_string())
    );

    let database_url = env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&database_url)
        .await
        .expect("数据库连接失败");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("数据库迁移失败");

    let api_context = ApiContext { config, pool };

    let listener = tokio::net::TcpListener::bind(url)
        .await
        .expect("服务启动失败");

    let app = app::api_router().await;

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Clone)]
struct ApiContext {
    config: HashMap<String, String>,
    pool: sqlx::PgPool,
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn run_migration() {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("数据库连接地址未设置");
        let pool = sqlx::PgPool::connect(&database_url)
            .await
            .expect("数据库连接失败");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("数据库迁移失败");
    }
}
