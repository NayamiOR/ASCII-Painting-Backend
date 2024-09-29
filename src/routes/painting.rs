use crate::dao::{
    delete_painting_by_id, get_painting_by_id, painting_exist, save_painting, Painting,
    PaintingState,
};
use crate::error::{Error, ServerError};
use crate::utils::authentication::*;
use crate::ApiContext;
use axum::extract::{Json, Query, State};
use axum::http::HeaderMap;
use axum::routing::{delete, get, post};
use axum::Router;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use sqlx::pool;
use sqlx::types::chrono;

pub fn create_route() -> Router<ApiContext> {
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
async fn get_painting(
    State(state): State<ApiContext>,
    Query(id): Query<String>,
) -> Json<GetPaintingResponse> {
    todo!("获取画作列表函数")
}

#[debug_handler]
async fn create_painting(
    State(state): State<ApiContext>,
    header_map: HeaderMap,
    Json(payload): Json<CreatePaintingRequest>,
) -> Result<Json<CreatePaintingResponse>, Error> {
    let CreatePaintingRequest { name, content } = payload;
    let jwt = header_map
        .get("Authorization")
        .ok_or(Error::Server(Box::new(ServerError::Unauthorized)))?
        .to_str()
        .map_err(|_| Error::Server(Box::new(ServerError::Unauthorized)))?;
    if !validate_jwt(jwt) {
        return Err(Error::Server(Box::new(ServerError::Unauthorized)));
    }

    let author_id = extract_claims(jwt).unwrap().id;
    let painting = Painting {
        id: 0,
        name,
        content,
        created_at: chrono::Utc::now().naive_utc(),
        author_id,
        favorite_num: 0,
        like_num: 0,
        state: PaintingState::Unreviewed,
    };
    save_painting(&state.pool, painting).await?;
    let response = Json(CreatePaintingResponse {
        message: "success".to_string(),
    });
    Ok(response)
}

#[debug_handler]
async fn delete_painting(
    State(state): State<ApiContext>,
    header_map: HeaderMap,
    Query(id): Query<i32>,
) -> Result<Json<DeletePaintingResponse>, Error> {
    // check if the painting exists
    let pool = &state.pool;
    if painting_exist(pool, id).await? {
        todo!()
    }

    // check jwt
    if !validate_jwt(header_map.get("Authorization").unwrap().to_str().unwrap()) {
        return Err(Error::Server(Box::new(ServerError::InvalidJwt)));
    }
    let author_id = extract_claims_from_header(header_map)?.id;

    // check if the painting belongs to the user
    let painting = get_painting_by_id(pool, id).await?;
    if painting.author_id != author_id {
        return Err(Error::Server(Box::new(ServerError::InvalidDelete)));
    }

    // delete the painting
    delete_painting_by_id(pool, id).await?;
    let response = DeletePaintingResponse {
        message: "success".to_string(),
    };
    Ok(Json(response))
}

#[debug_handler]
async fn like_painting(
    State(state): State<ApiContext>,
    Json(payload): Json<LikePaintingRequest>,
) -> Json<LikePaintingResponse> {
    todo!("获取画作列表函数")
}

#[debug_handler]
async fn favorite_painting(
    State(state): State<ApiContext>,
    Json(payload): Json<FavoritePaintingRequest>,
) -> Json<FavoritePaintingResponse> {
    todo!("收藏画作函数")
}

#[debug_handler]
async fn pass_painting(
    State(state): State<ApiContext>,
    Json(payload): Json<PassPaintingRequest>,
) -> Json<PassPaintingResponse> {
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
