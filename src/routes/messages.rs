use axum::Router;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use axum_macros::debug_handler;
use axum::extract::{Json, Query, State};
use crate::ApiContext;
use crate::models::*;

pub fn create_route() -> Router<ApiContext> {
    Router::new().nest(
        "/messages",
        Router::new()
            .route("/", get(get_messages))
            .route("/", post(delete_messages)),
    )
}

#[debug_handler]
async fn get_messages(State(state): State<ApiContext>) -> Json<GetMessagesResponse> {
    todo!("获取消息列表函数")
}

#[debug_handler]
async fn delete_messages(State(state): State<ApiContext>, Json(payload): Json<DeleteMessagesRequest>) -> Json<DeleteMessagesResponse> {
    todo!("删除消息函数")
}
