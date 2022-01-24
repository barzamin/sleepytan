use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTempl {
    message: String,
}

impl ErrorTempl {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Debug)]
pub enum AppError {
    Db(sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_resp) = match self {
            AppError::Db(err) => {
                // TODO
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))
            }
        };

        let body = Html(ErrorTempl::new(err_resp).render().unwrap());

        (status, body).into_response()
    }
}
