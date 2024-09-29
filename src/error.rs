use axum::response::IntoResponse;
use axum::Json;
use std::fmt::Display;

#[derive(Debug)]
pub(crate) enum Error {
    Database(Box<sqlx::Error>),
    Server(Box<ServerError>),
    Io(Box<std::io::Error>),
    Other(Box<dyn std::error::Error>),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        if matches!(self, Error::Server(_)) {
            return match self {
                Error::Server(e) => e.into_response(),
                _ => unreachable!(),
            };
        }
        let message = "Server error";
        let body = Json(serde_json::json!({
            "message": message,
        }));
        body.into_response()
    }
}

#[derive(Debug)]
pub(crate) enum ServerError {
    NoUser,
    UserAlreadyExists,
    PasswordIncorrect,
    NoPainting,
    Unauthorized,
    InvalidJwt,
    InvalidRefreshToken,
    InvalidDelete,
    Other(Box<dyn std::error::Error>),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        let message = match self {
            Self::NoUser => "No such user",
            Self::PasswordIncorrect => "Password incorrect",
            Self::UserAlreadyExists => "User already exists",
            Self::NoPainting => "No such painting",
            Self::Unauthorized => "Unauthorized",
            Self::InvalidJwt => "Invalid JWT",
            Self::InvalidRefreshToken => "Invalid refresh token",
            Self::InvalidDelete => "Invalid delete",

            Self::Other(e) => &format!("Unknown error: {}", e),
        };
        let body = Json(serde_json::json!({
            "message": message,
        }));
        body.into_response()
    }
}

#[derive(Debug)]
pub(crate) enum PaintingError {}
