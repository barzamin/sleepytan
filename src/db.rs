use color_eyre::eyre::Result;
use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
pub struct Handle {
    pub id: i64,
    pub accessor: i64,
    pub name: String,
}

pub async fn get_handle(pool: &SqlitePool, id: i64) -> Result<Option<Handle>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM handle WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}
