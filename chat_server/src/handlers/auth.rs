use crate::error::AppError;
use crate::models::{CreateUser, SignUser};
use crate::{AppState, User};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
// signin

pub async fn signup_handler(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::create(&input, &state.pool).await?;
    let token = state.ek.generate(&user)?;
    Ok((StatusCode::CREATED, token))
}

// signup
pub async fn signin_handler(
    State(state): State<AppState>,
    Json(input): Json<SignUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::verify(&input, &state.pool).await?;
    let token = state.ek.generate(&user)?;
    Ok((StatusCode::CREATED, token))
}

#[cfg(test)]

mod tests {
    use anyhow::Result;
    #[tokio::test]
    async fn test_signup() -> Result<()> {
        Ok(())
    }
}
