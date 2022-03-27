use argon2::PasswordHasher;
use password_hash::PasswordHash;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::data::Board;

pub type Pool = SqlitePool;
type Result<T> = std::result::Result<T, sqlx::Error>;

#[derive(Debug, sqlx::FromRow)]
pub struct Handle {
    pub id: Uuid,
    pub name: String,
    pub passhash: String,
    pub desc: String,
}

impl PartialEq for Handle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Handle {}

pub async fn insert_handle<'a>(
    pool: &'a SqlitePool,
    name: impl AsRef<str>,
    passhash: &'a PasswordHash<'a>,
) -> Result<Uuid> {
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO handle (id, passhash, `name`) VALUES (?, ?, ?)")
        .bind(id)
        .bind(passhash.to_string())
        .bind(name.as_ref())
        .execute(pool)
        .await?;
    Ok(id)
}

pub async fn get_handle(pool: &SqlitePool, id: Uuid) -> Result<Option<Handle>> {
    sqlx::query_as("SELECT * FROM handle WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn get_boards(pool: &SqlitePool) -> Result<Vec<Board>> {
    sqlx::query_as("SELECT * FROM board").fetch_all(pool).await
}

pub async fn get_board(pool: &SqlitePool, code: impl AsRef<str>) -> Result<Option<Board>> {
    sqlx::query_as("SELECT * FROM board WHERE code=?")
        .bind(code.as_ref())
        .fetch_optional(pool)
        .await
}
