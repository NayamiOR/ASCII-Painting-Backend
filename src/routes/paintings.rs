use axum::Router;
use axum_macros::debug_handler;
use axum::extract::{Json, Query, State};
use serde::{Deserialize, Serialize};
use axum::routing::{get};
use crate::{dao::{PaintingSort, PaintingState}, ApiContext};
use crate::dao::PaintingFilter;
use crate::models::*;

pub fn create_route() -> Router<ApiContext>{
    Router::new().nest(
        "/paintings",
        Router::new()
            .route("/all", get(get_paintings))
            .route("/user", get(get_user_paintings)),
    )
}

#[debug_handler]
async fn get_paintings(State(state):State<ApiContext>,Query(page): Query<String>,Query(sort):Query<PaintingSort>,Query(painting_state):Query<PaintingState>) -> Json<GetPaintingsResponseData> {
    // todo: 沟通API后加上时间限制
    // let painting_filter = PaintingFilter{
    //     page: page.parse().unwrap(),
    //     sort: sort,
    //     painting_state: painting_state,
    // };
    todo!("获取画作列表函数")
}

#[debug_handler]
async fn get_user_paintings(State(state):State<ApiContext>,Query(id): Query<String>) -> Json<GetUserPaintingsResponse> {
    todo!("获取用户画作列表函数")
}

