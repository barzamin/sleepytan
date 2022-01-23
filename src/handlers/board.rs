use axum::{response::Html};
use askama::Template;
use crate::data::Post;

#[derive(Template)]
#[template(path = "board.html")]
struct BoardTempl {
    posts: Vec<Post>,
}

pub async fn board() -> Html<String> {
    let hewwo = BoardTempl {
        posts: vec![Post {
            subject: "/sleepgen/".to_string(),
            text: "uwu".to_string(),
        }],
    };

    Html(hewwo.render().unwrap())
}
