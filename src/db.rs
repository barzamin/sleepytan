use argon2::PasswordHasher;
use password_hash::PasswordHash;
use sqlx::SqlitePool;

pub type Pool = SqlitePool;
type Result<T> = std::result::Result<T, sqlx::Error>;

#[derive(Debug, sqlx::FromRow)]
pub struct Handle {
    pub id: i64,
    pub accessor: i64,
    pub name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Accessor {
    pub id: i64,
    pub passhash: String,
}

pub async fn insert_accessor<'a>(
    pool: &'a SqlitePool,
    passhash: &'a PasswordHash<'a>,
) -> Result<i64> {
    Ok(sqlx::query("INSERT INTO accessor (passhash) VALUES (?)")
        .bind(passhash.to_string())
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

pub async fn get_accessor(pool: &SqlitePool, aid: i64) -> Result<Option<Accessor>> {
    sqlx::query_as("SELECT * FROM accessor WHERE id = ?")
        .bind(aid)
        .fetch_optional(pool)
        .await
}
