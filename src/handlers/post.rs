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

    let thread = sqlx::query!(
        "INSERT INTO `thread` (board, subject) VALUES (?, ?)",
        board.id,
        form.subject
    )
    .execute(&pool)
    .await?
    .last_insert_rowid();

    sqlx::query!(
        "INSERT INTO `post` (handle, thread, body) VALUES (?, ?, ?)",
        hctx.id,
        thread,
        form.body
    )
    .execute(&pool)
    .await?;

    Ok(Redirect::to(format!("/{}/", board.code).parse().unwrap()))
}

#[derive(Deserialize)]
pub struct ReplyForm {
    body: String,
}

pub async fn reply(
    hctx: Handle,
    Form(form): Form<ReplyForm>,
    Path(board_code): Path<String>,
    Path(post_id): Path<i64>,
    Extension(pool): Extension<db::Pool>,
) -> Result<Redirect, AppError> {
    debug!(%board_code, %post_id, "attempting reply");

    Ok(Redirect::to(Uri::from_static("/")))
}
