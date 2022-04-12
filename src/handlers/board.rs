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
    subject: String,
    body: String,
    create_ts: DateTime<Utc>,
    handle_name: String,
    handle_id: Uuid,
}

#[derive(Template)]
#[template(path = "board.html")]
struct BoardTempl {
    board: Board,
    posts: Vec<FEPost>,
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
//         let posts = sqlx::query_as!(FEPost, r#"
// SELECT post.subject, post.body, post.create_ts as "create_ts: _", handle.name as handle_name, handle.id as "handle_id!: _"
// FROM `post`
// INNER JOIN `handle` ON
//   `handle`.id = `post`.handle
// WHERE `post`.`board` = ?;"#, board.id)
//             .fetch_all(&pool)
//             .await?;
        let posts = vec![];

        let templ = BoardTempl {
            board,
            posts, /* vec![Post {
                       subject: "anyone noticed hyperpop kinda fruity".to_string(),
                       text: "s6e21 turn up troon out by leroy and blackwinterwells".to_string(),
                   }], */
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
