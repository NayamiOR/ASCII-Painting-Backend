use axum::extract::{Json, Query};
use axum::routing::{delete, get, post};
use axum::Router;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

pub fn create_route() -> Router {
    Router::new().nest(
        "/painting",
        Router::new()
            .route("/", get(get_painting))
            .route("/", post(create_painting))
            .route("/", delete(delete_painting))
            .route("/like", post(like_painting))
            .route("/favorite", post(favorite_painting))
            .route("/pass", post(pass_painting)),
    )
}

#[debug_handler]
async fn get_painting(Query(id): Query<String>) -> Json<GetPaintingResponse> {
    todo!("获取画作列表函数")
}

#[debug_handler]
async fn create_painting(Json(payload): Json<CreatePaintingRequest>) -> Json<CreatePaintingResponse> {
    todo!("创建画作函数")
}

#[debug_handler]
async fn delete_painting(Query(id): Query<i64>) -> Json<DeletePaintingResponse> {
    todo!("删除画作函数")
}

#[debug_handler]
async fn like_painting(Json(payload): Json<LikePaintingRequest>) -> Json<LikePaintingResponse> {
    todo!("获取画作列表函数")
}

#[debug_handler]
async fn favorite_painting(Json(payload): Json<FavoritePaintingRequest>) -> Json<FavoritePaintingResponse> {
    todo!("收藏画作函数")
}

#[debug_handler]
async fn pass_painting(Json(payload): Json<PassPaintingRequest>) -> Json<PassPaintingResponse> {
    todo!("通过画作函数")
}

#[derive(Serialize, Deserialize)]
struct GetPaintingResponse {
    pub id: i64,
    pub name: String,
    pub avatar: String,
}

#[derive(Serialize, Deserialize)]
struct Data {
    pub id: i64,
    pub name: String,
    pub content: String,
    pub favorite_num: i64,
    pub like_num: i64,
    pub time: String,
    pub author: GetPaintingResponse,
}

#[derive(Serialize, Deserialize)]
struct CreatePaintingRequest {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
struct CreatePaintingResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
struct DeletePaintingResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
struct LikePaintingRequest {
    pub painting_id: i64,
    pub state: String,
}

#[derive(Serialize, Deserialize)]
struct LikePaintingResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
struct FavoritePaintingRequest {
    pub painting_id: i64,
    pub state: String,
}

#[derive(Serialize, Deserialize)]
struct FavoritePaintingResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
struct PassPaintingRequest {
    pub id: i64,
    pub state: i64,
}

#[derive(Serialize, Deserialize)]
struct PassPaintingResponse {
    pub message: String,
}