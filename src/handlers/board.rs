use crate::{
    data::Board,
    db::Handle,
    err::AppError,
    templ::TemplCommon,
};
use askama::Template;
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::db;

struct FEPost {
    body: String,
    create_ts: DateTime<Utc>,
    handle_name: String,
    handle_id: Uuid,
}

struct FEThread {
    subject: String,
    posts: Vec<FEPost>,
}

#[derive(Template)]
#[template(path = "board.html")]
struct BoardTempl {
    board: Board,
    // posts: Vec<FEPost>,
    threads: Vec<FEThread>,
    common: TemplCommon,
}

#[derive(Template)]
#[template(path = "board_404.html")]
struct Board404 {
    common: TemplCommon,
}

pub async fn get(
    hctx: Option<Handle>,
    Path(code): Path<String>,
    Extension(pool): Extension<db::Pool>,
) -> Result<impl IntoResponse, AppError> {
    let board = db::get_board(&pool, code).await?;

    if let Some(board) = board {
        let threads = {
            // todo: limit and sort
            let mut threads = vec![];
            for thread in sqlx::query!(r#"SELECT * FROM threads WHERE board = ?"#, board.id).fetch_all(&pool).await? {
                threads.push(FEThread {
                    subject: thread.subject,
                    posts: sqlx::query_as!(FEPost, r#"
SELECT post.body, post.create_ts as "create_ts: _", handle.name as handle_name, handle.id as "handle_id!: _"
FROM post
INNER JOIN handle ON
    handle.id = post.handle
WHERE
    post.parent = ? OR post.id = ?"#, thread.id, thread.id).fetch_all(&pool).await?
                });
            }
            threads
        };

        let templ = BoardTempl {
            board,
            threads,
            common: TemplCommon { hctx },
        };

        Ok(Html(templ.render().unwrap()).into_response())
    } else {
        Ok((
            StatusCode::NOT_FOUND,
            Html(
                Board404 {
                    common: TemplCommon { hctx },
                }
                .render()
                .unwrap(),
            ),
        )
            .into_response())
    }
}
