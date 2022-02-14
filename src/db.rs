use argon2::PasswordHasher;
use password_hash::PasswordHash;
use sqlx::SqlitePool;

use crate::data::Board;

pub type Pool = SqlitePool;
type Result<T> = std::result::Result<T, sqlx::Error>;

#[derive(Debug, sqlx::FromRow)]
pub struct Handle {
    pub id: i64,
    pub name: String,
    pub passhash: String,
}

pub async fn insert_handle<'a>(
    pool: &'a SqlitePool,
    name: impl AsRef<str>,
    passhash: &'a PasswordHash<'a>,
) -> Result<i64> {
    Ok(sqlx::query("INSERT INTO handle (passhash, `name`) VALUES (?, ?)")
        .bind(passhash.to_string())
        .bind(name.as_ref())
        .execute(pool)
        .await?
        .last_insert_rowid())
}

pub async fn get_handle(pool: &SqlitePool, id: i64) -> Result<Option<Handle>> {
    sqlx::query_as("SELECT * FROM handle WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn get_boards(pool: &SqlitePool) -> Result<Vec<Board>> {
    sqlx::query_as("SELECT * FROM board")
        .fetch_all(pool)
        .await
}
