use config::{Config, File};
use serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::Connection;
use std::env;

struct AppConfig {
    url: String,
    pool_size: u32,
}

impl AppConfig {
    pub async fn new() -> AppConfig {
        let host = env::var("HOST").expect("HOST MUST BE SET");
        let url = env::var("DATABASE_URL").expect("DATABASE_URL MUST BE SET");
        // // test url
        let _ = sqlx::postgres::PgConnection::connect(&url)
            .await
            .expect("DATABASE CONNECTION ERROR, PLEASE CHECK DATABASE URL");

        let pool_size: u32 = match env::var("POOL_SIZE") {
            Ok(pool_size) => pool_size.parse().expect("POOL_SIZE MUST BE A NUMBER"),
            Err(_) => 10,
        };

        dbg!(&url);
        AppConfig { url, pool_size }
    }
}
