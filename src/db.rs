use argon2::PasswordHasher;
use color_eyre::eyre::Result;
use password_hash::PasswordHash;
use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
pub struct Handle {
    pub id: i64,
    pub accessor: i64,
    pub name: String,
}

pub async fn insert_accessor<'a>(
    pool: &'a SqlitePool,
    passhash: &'a PasswordHash<'a>,
) -> Result<i64, sqlx::Error> {
    Ok(sqlx::query("INSERT INTO accessor (passhash) VALUES (?)")
        .bind(passhash.to_string())
        .execute(pool)
        .await?
        .last_insert_rowid())
}

pub async fn get_handle(pool: &SqlitePool, id: i64) -> Result<Option<Handle>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM handle WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}
