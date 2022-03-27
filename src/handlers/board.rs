use crate::{data::{Post, Board}, db::Handle, templ::TemplCommon, err::AppError};
use askama::Template;
use axum::{
    extract::{Extension, Path},
    response::Html,
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
#[template(path="board_404.html")]
struct Board404 {
    common: TemplCommon,
}

pub async fn get(
    hctx: Option<Handle>,
    Path(code): Path<String>,
    Extension(pool): Extension<db::Pool>,
) -> Result<Html<String>, AppError> {
    let board = db::get_board(&pool, code).await?;

    if let Some(board) = board {
        let templ = BoardTempl {
            board: board,
            posts: vec![Post {
                subject: "/sleepgen/".to_string(),
                text: "uwu".to_string(),
            }],
            common: TemplCommon { hctx },
        };

        Ok(Html(templ.render().unwrap()))
    } else {
        Ok(Html(Board404 { common: TemplCommon { hctx } }.render().unwrap()))
    }
}
