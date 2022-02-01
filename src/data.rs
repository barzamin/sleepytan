use sqlx::FromRow;

#[derive(Debug)]
pub struct Post {
    pub subject: String,
    pub text: String,
}

#[derive(Debug, FromRow)]
pub struct Board {
    pub code: String,
    pub desc: String,
}
