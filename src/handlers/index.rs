use askama::Template;
use axum::{extract::Extension, response::Html};

use crate::data::Board;
use crate::db::{self, Handle};
use crate::err::AppError;
use crate::templ::TemplCommon;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTempl {
    common: TemplCommon,
    boards: Vec<Board>,
}

pub async fn get(
    hctx: Option<Handle>,
    Extension(pool): Extension<db::Pool>,
) -> Result<Html<String>, AppError> {
    let boards = db::get_boards(&pool).await?;

    let templ = IndexTempl {
        boards,
        common: TemplCommon { hctx },
    };

    Ok(Html(templ.render().unwrap()))
}
