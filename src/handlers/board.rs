use crate::{data::Post, db::Handle, templ::TemplCommon};
use askama::Template;
use axum::{
    extract::{Extension, Path},
    response::Html,
};

use crate::db;

#[derive(Template)]
#[template(path = "board.html")]
struct BoardTempl {
    posts: Vec<Post>,
    common: TemplCommon,
}

pub async fn get(
    hctx: Option<Handle>,
    Path(code): Path<String>,
    Extension(pool): Extension<db::Pool>,
) -> Html<String> {
    let templ = BoardTempl {
        posts: vec![Post {
            subject: "/sleepgen/".to_string(),
            text: "uwu".to_string(),
        }],
        common: TemplCommon { hctx },
    };

    Html(templ.render().unwrap())
}
