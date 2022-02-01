use askama::Template;
use axum::{response::Html, extract::Extension};

use crate::data::Board;
use crate::db;
use crate::err::AppError;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTempl {
    boards: Vec<Board>,
}

pub async fn get(
    Extension(pool): Extension<db::Pool>
) -> Result<Html<String>, AppError> {
    let boards = db::get_boards(&pool).await?;

    let templ = IndexTempl {
        boards
    };

    Ok(Html(templ.render().unwrap()))
}
