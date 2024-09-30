use crate::dao::{self, get_user_by_email, save_user, user_email_exist, User};
use crate::error::{Error, ServerError};
use crate::models::*;
use crate::utils::authentication::{
    extract_claims_from_header, generate_jwt, generate_refresh_token,
};
use crate::ApiContext;
use axum::extract::{Json, Query, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use std::env;
use std::fmt::Debug;

pub fn create_route() -> Router<ApiContext> {
    Router::new().nest(
        "/user",
        Router::new()
            .route("/login", post(login))
            .route("/register", post(register))
            .route("/info", get(info))
            .route("/info", post(update_info))
            .route("/avatar", post(update_avatar)),
        // .route("/code", post(get_code)),
    )
}

#[debug_handler]
async fn login(
    State(state): State<ApiContext>,
    Json(payload): Json<LoginRequest>,
) -> Result<(HeaderMap, Json<LoginResponse>), Error> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL MUST BE SET");
    let pool = &state.pool;

    if !user_email_exist(pool, &payload.email).await? {
        return Err(Error::Server(Box::new(ServerError::NoUser)));
    }
    // todo: 统一表达（同register）
    // todo: 修改错误匹配
    let user = match get_user_by_email(pool, &payload.email).await {
        Ok(user) => user,
        Err(_) => return Err(Error::Server(Box::new(ServerError::NoUser))),
    };

    if !bcrypt::verify(&payload.password, &user.password).unwrap() {
        return Err(Error::Server(Box::new(ServerError::PasswordIncorrect)));
    }
    let jwt = generate_jwt(&payload.email, user.id);
    let refresh_token = generate_refresh_token();

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", jwt).parse().unwrap());

    let body = Json(LoginResponse {
        message: "Login success".to_string(),
        refresh_token,
    });

    Ok((headers, body))
}

#[debug_handler]
async fn register(
    State(state): State<ApiContext>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(HeaderMap, Json<RegisterResponse>), ServerError> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL MUST BE SET");
    let pool = &state.pool;

    // 检查用户是否已经存在
    match user_email_exist(pool, &payload.email).await {
        Ok(true) => return Err(ServerError::UserAlreadyExists),
        Err(e) => return Err(ServerError::Other(Box::new(e))),
        _ => {}
    }

    // 保存用户
    let user = User {
        id: 0,
        username: payload.name.clone(),
        email: payload.email.clone(),
        created_at: Utc::now().naive_utc(),
        password: payload.password.clone(),
    };

    save_user(pool, &user).await.unwrap();

    // 生成jwt和refresh token，返回
    let jwt = generate_jwt(&payload.email, user.id);
    let refresh_token = generate_refresh_token();

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", jwt).parse().unwrap());

    let body = Json(RegisterResponse {
        message: "Register success".to_string(),
        refresh_token,
    });

    Ok((headers, body))
}

#[debug_handler]
async fn info(State(state): State<ApiContext>, Query(id): Query<i64>) -> Json<UserInfoResponse> {
    todo!("获取用户信息函数")
}

#[debug_handler]
async fn info_by_email(
    State(state): State<ApiContext>,
    Query(email): Query<String>,
) -> Result<Json<UserInfoResponse>, ServerError> {
    let pool = &state.pool;
    let user = get_user_by_email(pool, &email)
        .await
        .map_err(|_| ServerError::NoUser)?;
    let result = UserInfoResponse {
        message: "success".to_string(),
        data: UserInfoResponseData {
            id: user.id,
            name: user.username,
            avatar: todo!(),
            work_num: 0,
            like_num: 0,
            favorite_num: 0,
        },
    };
    todo!()
}

#[debug_handler]
async fn update_info(
    State(state): State<ApiContext>,
    Json(payload): Json<UpdateUserInfoRequest>,
) -> Json<UpdateUserInfoResponse> {
    // let { id, name, avatar, password, level } = payload;
    let UpdateUserInfoRequest { name } = payload;
    let pool = &state.pool;
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
async fn update_avatar(
    State(state): State<ApiContext>,
    header: HeaderMap,
    Json(payload): Json<UpdateAvatarRequest>,
) -> Result<Json<UpdateAvatarResponse>, Error> {
    // extract user id from jwt
    let pool = &state.pool;
    let jwt = header.get("Authorization").unwrap().to_str();
    match jwt {
        Ok(jwt) => {
            let user_id = extract_claims_from_header(header)?.id;
        }
        Err(_) => return Err(Error::Server(Box::new(ServerError::Unauthorized))),
    }

    let new_avatar = payload.avatar.clone();
    todo!("更新用户头像函数")
}

// #[debug_handler]
// async fn get_code(Json(payload): Json<CodeRequest>) -> Json<CodeResponse> {
//     todo!("获取验证码函数")
// }
