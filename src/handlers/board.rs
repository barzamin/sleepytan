use crate::{
    data::{Board, Post},
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

use crate::db;

#[derive(Template)]
#[template(path = "board.html")]
struct BoardTempl {
    board: Board,
    posts: Vec<Post>,
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
        let templ = BoardTempl {
            board: board,
            posts: vec![Post {
                subject: "anyone noticed hyperpop kinda fruity".to_string(),
                text: "s6e21 turn up troon out by leroy and blackwinterwells".to_string(),
            }],
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
