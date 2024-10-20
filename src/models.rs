use crate::dao::PaintingState;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GetPaintingResponse {
    pub id: i64,
    pub name: String,
    // todo: avatar?
    pub avatar: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CreatePaintingRequest {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CreatePaintingResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DeletePaintingResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct LikePaintingRequest {
    pub painting_id: i64,
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct LikePaintingResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct FavoritePaintingRequest {
    pub painting_id: i64,
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct FavoritePaintingResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PassPaintingRequest {
    pub id: i64,
    pub state: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PassPaintingResponse {
    pub message: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub(crate) struct LoginResponse {
    pub(crate) message: String,
    pub(crate) user: UserBasicInfoResponse,
}

#[derive(Serialize, Debug, Deserialize)]
pub(crate) struct UserBasicInfoResponse {
    pub(crate) user_id: i32,
    pub(crate) name: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct LoginRequest {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Serialize, Debug)]
pub(crate) struct RegisterResponse {
    pub(crate) message: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct RegisterRequest {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) password: String,
    // 暂时不实现
    // code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UserInfoResponseData {
    pub id: i32,
    pub name: String,
    pub avatar: String,
    pub work_num: i64,
    pub like_num: i64,
    pub favorite_num: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UserInfoResponse {
    pub message: String,
    pub data: UserInfoResponseData,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UpdateUserInfoResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UpdateUserInfoRequest {
    // todo
    pub name: String,
}

// multipart/form-data includes a optional file field
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UpdateAvatarRequest {
    pub avatar: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UpdateAvatarResponse {
    pub avatar: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UpdateAvatarResponseData {
    pub message: String,
    pub data: UpdateAvatarResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CodeRequest {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CodeResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GetPaintingsResponse {
    pub message: String,
    pub data: Vec<PaintingData>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct PaintingData {
    pub id: i32,
    pub name: String,
    pub content: String,
    pub favorite_num: i32,
    pub like_num: i32,
    // pub browse_num: i32,
    pub state: PaintingState,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GetUserPaintingsResponse {}

#[derive(Serialize, Deserialize)]
pub(crate) struct GetMessagesResponse {}

#[derive(Serialize, Deserialize)]
pub(crate) struct DeleteMessagesRequest {}

#[derive(Serialize, Deserialize)]
pub(crate) struct DeleteMessagesResponse {}
