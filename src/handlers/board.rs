use crate::data::Post;
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
}

pub async fn get(Path(code): Path<String>, Extension(pool): Extension<db::Pool>) -> Html<String> {
    let templ = BoardTempl {
        posts: vec![Post {
            subject: "/sleepgen/".to_string(),
            text: "uwu".to_string(),
        }],
    };

    Html(templ.render().unwrap())
}
