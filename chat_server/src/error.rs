use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use jwt_simple::reexports::serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("password hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("jwt error: {0}")]
    JwtError(#[from] jwt_simple::Error),

    #[error("password is not correct")]
    PasswordWrongError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            AppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PasswordHashError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::JwtError(_) => StatusCode::FORBIDDEN,
            AppError::PasswordWrongError => StatusCode::FORBIDDEN,
        };
        (status, Json(json!({ "error": self.to_string()}))).into_response()
    }
}
