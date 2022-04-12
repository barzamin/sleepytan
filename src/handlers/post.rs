use axum::{
    extract::{Extension, Form, Path},
    http::Uri,
    response::Redirect,
};
use color_eyre::eyre::eyre;
use serde::Deserialize;
use tracing::debug;

use crate::{
    data::Board,
    db::{self, Handle},
    err::AppError,
};

#[derive(Deserialize)]
pub struct PostForm {
    subject: String,
    body: String,
}

pub async fn create_post(
    hctx: Handle,
    Form(form): Form<PostForm>,
    Path(board_code): Path<String>,
    Extension(pool): Extension<db::Pool>,
) -> Result<Redirect, AppError> {
    debug!(%board_code, "attempting post to board");
    let board: Board = sqlx::query_as("SELECT * FROM board WHERE code = ?")
        .bind(board_code)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::GenericISE(eyre!("can't post to a nonexistent board")))?;

    sqlx::query("INSERT INTO `post` (handle, board, subject, body) VALUES (?, ?, ?, ?)")
        .bind(hctx.id)
        .bind(board.id)
        .bind(form.subject)
        .bind(form.body)
        .execute(&pool)
        .await?;

    Ok(Redirect::to(format!("/{}/", board.code).parse().unwrap()))
}
