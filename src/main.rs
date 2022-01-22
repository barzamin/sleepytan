use axum::{routing::get, response::Html, Router};
use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]
struct HewwoTempl {
    msg: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<String> {
    let hewwo = HewwoTempl { msg: "uwu".to_owned() };
    Html(hewwo.render().unwrap())
}
