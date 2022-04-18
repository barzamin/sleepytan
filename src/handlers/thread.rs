use askama::Template;
use axum::{
    extract::{Extension, Path, Form},
    response::{Html, IntoResponse, Redirect}, http::StatusCode,
};
use chrono::{DateTime, Utc};
use color_eyre::eyre::eyre;
use serde::Deserialize;
use tracing::debug;
use uuid::Uuid;

use crate::{
    db::{self, Handle},
    err::AppError,
    templ::TemplCommon, data::Board,
};

struct FEPost {
    id: i64,
    body: String,
    create_ts: DateTime<Utc>,
    handle_name: String,
    handle_id: Uuid,
}

struct FEThread {
    id: i64,
    subject: String,
    posts: Vec<FEPost>,
}

#[derive(Template)]
#[template(path = "thread.html")]
struct ThreadTempl {
    thread: FEThread,
    common: TemplCommon,
}

#[derive(Template)]
#[template(path = "thread_404.html")]
struct Thread404Templ {
    common: TemplCommon,
}

#[derive(Deserialize)]
pub struct PostForm {
    subject: String,
    body: String,
}

#[derive(Deserialize)]
pub struct ReplyForm {
    body: String,
}

pub async fn create(
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

pub async fn get(
    hctx: Option<Handle>,
    Path(id): Path<i64>,
    Extension(pool): Extension<db::Pool>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(thread) = sqlx::query!("SELECT * FROM thread WHERE id = ?", id)
        .fetch_optional(&pool)
        .await?
    {
        let thread = FEThread {
            id: thread.id,
            subject: thread.subject,
            posts: sqlx::query_as!(FEPost, r#"
            SELECT post.id, post.body, post.create_ts as "create_ts: _", handle.name as handle_name, handle.id as "handle_id!: _"
            FROM post
            INNER JOIN handle ON
                handle.id = post.handle
            WHERE
                post.thread = ?"#, thread.id).fetch_all(&pool).await?
        };

        let templ = ThreadTempl {
            thread,
            common: TemplCommon { hctx },
        };
        Ok(Html(templ.render().unwrap()).into_response())
    } else {
        Ok((
            StatusCode::NOT_FOUND,
            Html(
                Thread404Templ {
                    common: TemplCommon { hctx },
                }
                .render()
                .unwrap(),
            ),
        )
            .into_response())
    }
}

pub async fn reply(
    hctx: Handle,
    Form(form): Form<ReplyForm>,
    Path(thread_id): Path<i64>,
    Extension(pool): Extension<db::Pool>,
) -> Result<Redirect, AppError> {
    debug!(%thread_id, "attempting reply to thread");

    sqlx::query!(
        "INSERT INTO `post` (handle, thread, body) VALUES (?, ?, ?)",
        hctx.id,
        thread_id,
        form.body
    )
    .execute(&pool)
    .await?;

    Ok(Redirect::to(format!("/_/{}", thread_id).parse().unwrap()))

}