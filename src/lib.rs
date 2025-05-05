mod config;
mod handlers;

use crate::handlers::{
    create_chat_handler, delete_chat_handler, list_chat_handler, list_message_handler,
    send_message_handler, signin_handler, signup_handler, update_chat_handler,
};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, patch, post};
use axum::Router;
pub use config::AppConfig;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

impl AppState {
    pub(crate) fn new(config: AppConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner { config }),
        }
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
}

pub fn get_router(config: AppConfig) -> Router {
    let state = AppState::new(config);
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
    Router::new()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state)
}

async fn index_handler(state: State<AppState>) -> impl IntoResponse {
    format!(
        "server started at: {}",
        state.config.server.port.to_string()
    )
}
