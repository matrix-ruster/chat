use crate::error::AppError;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, Utc};
use jwt_simple::prelude::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignUser {
    pub email: String,
    pub password: String,
}

impl User {
    /// 根据email获取用户
    pub async fn find_by_email(email: &str, pool: &sqlx::PgPool) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>("select * from users where email = $1")
            .bind(email)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }

    /// 添加用户
    pub async fn create(user_info: &CreateUser, pool: &sqlx::PgPool) -> Result<User, AppError> {
        let password_hash = hash_password(&user_info.password)?;
        let user = sqlx::query_as::<_, User>(
            "insert into users (username, email, password) values ($1,$2,$3) returning id, username, nickname, email, password, created_at",
        )
        .bind(&user_info.username)
        .bind(&user_info.email)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    /// 验证email和password
    pub async fn verify(sign_info: &SignUser, pool: &sqlx::PgPool) -> Result<User, AppError> {
        let user_info =
            sqlx::query_as::<_, User>("select email,password from users where email = $1")
                .bind(&sign_info.email)
                .fetch_one(pool)
                .await?;
        let is_match = verify_password(&sign_info.password, &*user_info.password)?;
        if is_match {
            Ok(user_info)
        } else {
            Err(AppError::PasswordWrongError)
        }
    }
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

fn verify_password(password: &str, hashed_password: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(hashed_password)?;

    // verify
    let is_match = argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok();
    Ok(is_match)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::PgPool;

    pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
        let _ = dotenvy::dotenv().is_ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
    }

    #[sqlx::test]
    async fn create_user_and_verify_test() -> sqlx::Result<(), AppError> {
        let pool = create_pool().await?;
        let u = User::create(
            &CreateUser {
                username: "johnsonsmile".to_string(),
                email: "johnsonsmile@163.com".to_string(),
                password: "123456".to_string(),
            },
            &pool,
        )
        .await?;
        assert_eq!(u.username, "johnsonsmile");
        dbg!("user: {:#?}", &u);
        let user = User::verify(
            &SignUser {
                email: "johnsonsmile@163.com".to_string(),
                password: "123456".to_string(),
            },
            &pool,
        )
        .await?;
        dbg!("user: {:#?}", &user);
        assert_eq!(user.email, u.email);
        let res = sqlx::query("delete from users where username = $1")
            .bind(u.username)
            .execute(&pool)
            .await?;
        assert_eq!(res.rows_affected(), 1);
        Ok(())
    }
}
