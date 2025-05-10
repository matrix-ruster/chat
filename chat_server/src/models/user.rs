use crate::error::AppError;
use crate::User;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

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
    pub async fn create(
        username: &str,
        email: &str,
        password: &str,
        pool: &sqlx::PgPool,
    ) -> Result<User, AppError> {
        let password_hash = hash_password(password)?;
        let user = sqlx::query_as::<_, User>(
            "insert into users (username, email, password) values ($1,$2,$3) returning id, username, nickname, email, password, created_at",
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    /// 验证email和password
    pub async fn verify(
        email: &str,
        password: &str,
        pool: &sqlx::PgPool,
    ) -> Result<bool, AppError> {
        let user_info =
            sqlx::query_as::<_, User>("select email,password from users where email = $1")
                .bind(email)
                .fetch_one(pool)
                .await?;
        verify_password(password, &*user_info.password)
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
        let u = User::create("johnsonsmile", "johnsonsmile@163.com", "123456", &pool).await?;
        assert_eq!(u.username, "johnsonsmile");
        dbg!("user: {:#?}", &u);
        let is_match = User::verify("johnsonsmile@163.com", "123456", &pool).await?;
        assert!(is_match);
        dbg!("is_match: {:#?}", is_match);
        let res = sqlx::query("delete from users where username = $1")
            .bind(u.username)
            .execute(&pool)
            .await?;
        assert_eq!(res.rows_affected(), 1);
        Ok(())
    }
}
