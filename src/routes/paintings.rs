use axum::Router;
use axum_macros::debug_handler;
use axum::extract::{Json, Query};
use serde::{Deserialize, Serialize};
use axum::routing::{get};

pub fn create_route() -> Router {
    Router::new().nest(
        "/paintings",
        Router::new()
            .route("/all", get(get_paintings))
            .route("/user", get(get_user_paintings)),
    )
}

#[debug_handler]
async fn get_paintings(Query(id): Query<String>) -> Json<GetPaintingsResponseData> {
    todo!("获取画作列表函数")
}

#[debug_handler]
async fn get_user_paintings(Query(id): Query<String>) -> Json<GetUserPaintingsResponse> {
    todo!("获取用户画作列表函数")
}

#[derive(Serialize, Deserialize)]
struct GetPaintingsResponseData {
    pub id: i64,
    pub name: String,
    pub content: String,
    pub favorite_num: i64,
    pub like_num: i64,
    pub state: i64,
}

#[derive(Serialize, Deserialize)]
struct GetPaintingsResponse {
    pub message: String,
    pub data: Vec<GetPaintingsResponseData>,
}

#[derive(Serialize, Deserialize)]
struct GetUserPaintingsResponse {}