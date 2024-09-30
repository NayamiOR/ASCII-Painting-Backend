use crate::dao::{filter_paintings, PaintingFilter};
use crate::error::Error;
use crate::models::*;
use crate::{
    dao::{PaintingSort, PaintingState},
    ApiContext,
};
use axum::extract::{Json, Query, State};
use axum::routing::get;
use axum::Router;
use axum_macros::debug_handler;
use sqlx::types::chrono::DateTime;

pub fn create_route() -> Router<ApiContext> {
    Router::new().nest(
        "/paintings",
        Router::new()
            .route("/all", get(get_paintings))
            .route("/user", get(get_user_paintings)),
    )
}

#[debug_handler]
async fn get_paintings(
    State(state): State<ApiContext>,
    Query(page): Query<String>,
    Query(sort): Query<PaintingSort>,
    Query(time): Query<String>, // RFC3339 格式的时间戳
    Query(painting_state): Query<PaintingState>,
) -> Result<Json<GetPaintingsResponse>, Error> {
    let pool = &state.pool;

    let painting_filter = PaintingFilter {
        page: Some(page.parse().unwrap()),
        sort: Some(sort),
        state: Some(painting_state),
        time: Some(DateTime::parse_from_rfc3339(&time).unwrap().into()),
    };

    let paintings = filter_paintings(pool, painting_filter).await?;
    let painting_data: Vec<PaintingData> = paintings
        .iter()
        .map(|painting| PaintingData {
            id: painting.id,
            name: painting.name.clone(),
            content: painting.content.clone(),
            state: painting.state.clone(),
            favorite_num: painting.favorite_num,
            like_num: painting.like_num,
        })
        .collect();

    let json = Json(GetPaintingsResponse {
        message: format!("Get {} paintings success", paintings.len()),
        data: painting_data,
    });

    Ok(json)
}

#[debug_handler]
async fn get_user_paintings(
    State(state): State<ApiContext>,
    Query(id): Query<String>,
) -> Json<GetUserPaintingsResponse> {
    todo!("获取用户画作列表函数")
}
