mod config;
mod error;
mod handlers;
mod models;
mod utils;

use crate::error::AppError;
use crate::handlers::{
    create_chat_handler, delete_chat_handler, list_chat_handler, list_message_handler,
    send_message_handler, signin_handler, signup_handler, update_chat_handler,
};
use anyhow::Context;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, patch, post};
use axum::Router;
pub use config::AppConfig;
pub use models::User;
use sqlx::postgres::PgPoolOptions;
use std::ops::Deref;
use std::sync::Arc;
pub use utils::{DecodingKey, EncodingKey};

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

impl AppState {
    pub(crate) async fn try_new(config: AppConfig) -> Result<Self, AppError> {
        let dk = DecodingKey::load(&config.auth.pk).context("load pk failed")?;
        let ek = EncodingKey::load(&config.auth.sk).context("load sk failed")?;
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.server.db_url)
            .await
            .context("connect to db failed")?;
        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                ek,
                dk,
                pool,
            }),
        })
    }
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug)]
pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) ek: EncodingKey,
    pub(crate) dk: DecodingKey,
    pub(crate) pool: sqlx::PgPool,
}

pub async fn try_init_router(config: AppConfig) -> Result<Router, AppError> {
    let state = AppState::try_new(config).await?;
    let api = Router::new()
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler))
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chat/{id}",
            patch(update_chat_handler).delete(delete_chat_handler),
        )
        .route(
            "/chat/{id}/messages",
            get(list_message_handler).post(send_message_handler),
        );
    Ok(Router::new()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state))
}

async fn index_handler(state: State<AppState>) -> impl IntoResponse {
    format!(
        "server started at: {}",
        state.config.server.port.to_string()
    )
}
