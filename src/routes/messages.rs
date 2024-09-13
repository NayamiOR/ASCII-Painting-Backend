use axum::Router;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use axum_macros::debug_handler;
use axum::extract::{Json, Query};

pub fn create_route() -> Router {
    Router::new().nest(
        "/messages",
        Router::new()
            .route("/",get(get_messages))
            .route("/", post(delete_messages)),
    )
}

#[debug_handler]
async fn get_messages() -> Json<GetMessagesResponse> {
    todo!("获取消息列表函数")
}

#[debug_handler]
async fn delete_messages(Json(payload): Json<DeleteMessagesRequest>) -> Json<DeleteMessagesResponse> {
    todo!("删除消息函数")
}

#[derive(Serialize, Deserialize)]
struct GetMessagesResponseData {
    pub id: i64,
    pub content: String,
    pub time: String,
}

#[derive(Serialize, Deserialize)]
struct GetMessagesResponse {
    pub message: String,
    pub data: Vec<GetMessagesResponseData>,
}

#[derive(Serialize, Deserialize)]
struct DeleteMessagesRequest {
    pub ids: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
struct DeleteMessagesResponse {
    pub message: String,
}