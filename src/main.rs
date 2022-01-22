use askama::Template;
use axum::{response::Html, routing::get, Router};

struct Post {
    subject: String,
    text: String,
}

#[derive(Template)]
#[template(path = "board.html")]
struct BoardTempl {
    posts: Vec<Post>,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<String> {
    let hewwo = BoardTempl {
        posts: vec![Post {
            subject: "/sleepgen/".to_string(),
            text: "uwu".to_string(),
        }],
    };

    Html(hewwo.render().unwrap())
}
