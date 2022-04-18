use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Post {
    pub body: String,
}

#[derive(Debug, FromRow)]
pub struct Board {
    pub id: i64,
    pub code: String,
    pub desc: String,
}
