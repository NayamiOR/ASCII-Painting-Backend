use axum::{Router};
use axum::extract::{Json, Query};
use axum::http::Response;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use axum_macros::debug_handler;

pub fn create_route() -> Router {
    Router::new().nest(
        "/user",
        Router::new()
            .route("/login", post(login))
            .route("/regist", post(register))
            .route("/info", get(info))
            .route("/info", post(update_info))
            .route("/avatar", post(update_avatar))
            .route("/code", post(get_code)),
    )
}

#[debug_handler]
async fn login(Json(payload): Json<LoginRequest>) -> Json<LoginResponse> {
    todo!("登录函数");
}

#[debug_handler]
async fn register(Json(payload): Json<RegisterRequest>) -> Json<RegisterResponse> {
    todo!("注册函数")
}

#[debug_handler]
async fn info(Query(id): Query<i64>) -> Json<UserInfoResponse> {
    todo!("获取用户信息函数")
}

#[debug_handler]
async fn update_info(Json(payload): Json<UpdateUserInfoRequest>) -> Json<UpdateUserInfoResponse> {
    todo!("更新用户信息函数")
}

// 两个设置相关函数暂时不实现
// #[debug_handler]
// async fn get_settings() -> Json<UpdateUserInfoResponse> {
//     todo!("获取用户设置函数")
// }
//
// #[debug_handler]
// async fn update_settings() -> Json<UpdateUserInfoResponse> {
//     todo!("更新用户设置函数")
// }

#[debug_handler]
async fn update_avatar() -> Json<UpdateUserInfoResponse> {
    todo!("更新用户头像函数")
}

#[debug_handler]
async fn get_code(Json(payload): Json<CodeRequest>) -> Json<CodeResponse> {
    todo!("获取验证码函数")
}

#[derive(Serialize)]
struct LoginResponse {
    message: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    message: String,
}

#[derive(Deserialize)]
struct RegisterRequest {
    name: String,
    email: String,
    password: String,
    code: String,
}

#[derive(Serialize, Deserialize)]
struct UserInfoResponseData {
    pub id: i64,
    pub name: String,
    pub avatar: String,
    pub work_num: i64,
    pub like_num: i64,
    pub favorite_num: i64,
}

#[derive(Serialize, Deserialize)]
struct UserInfoResponse {
    pub message: String,
    pub data: UserInfoResponseData,
}

#[derive(Serialize, Deserialize)]
struct UpdateUserInfoResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
struct UpdateUserInfoRequest {
    pub id: i64,
    pub name: String,
    pub avatar: String,
    pub password: String,
    pub level: String,
}

// multipart/form-data includes a optional file field
#[derive(Serialize, Deserialize)]
struct UpdateAvatarRequest {
    // todo
}

#[derive(Serialize, Deserialize)]
struct UpdateAvatarResponse {
    pub avatar: String,
}

#[derive(Serialize, Deserialize)]
struct UpdateAvatarResponseData {
    pub message: String,
    pub data: UpdateAvatarResponse,
}

#[derive(Serialize, Deserialize)]
struct CodeRequest {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
struct CodeResponse {
    pub message: String,
}