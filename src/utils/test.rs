use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub(crate) fn get_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub(crate) async fn init_pool() -> PgPool {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL MUST BE SET");
    PgPoolOptions::new()
        .max_connections(100)
        .connect(&url)
        .await
        .expect("Failed to create pool")
}
