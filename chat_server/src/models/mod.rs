mod user;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, Default)]
#[sqlx(default)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    #[serde(skip)]
    pub password: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}
