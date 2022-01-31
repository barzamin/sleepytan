use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use color_eyre::eyre;

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
    PwHash(password_hash::Error),
    Session(async_session::Error),
    GenericISE(eyre::Report),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::Db(err)
    }
}

impl From<password_hash::Error> for AppError {
    fn from(err: password_hash::Error) -> Self {
        Self::PwHash(err)
    }
}

impl From<async_session::Error> for AppError {
    fn from(err: async_session::Error) -> Self {
        Self::Session(err)
    }
}

impl From<eyre::Report> for AppError {
    fn from(err: eyre::Report) -> Self {
        Self::GenericISE(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_resp) = match self {
            Self::Db(err) => {
                // TODO
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("db error: {}", err),
                )
            }
            Self::PwHash(err) => {
                // TODO
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("hashing error: {}", err),
                )
            }
            Self::Session(err) => {
                // TODO
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("session error: {}", err),
                )
            }
            Self::GenericISE(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("ISE: {}", err)),
        };

        let body = Html(ErrorTempl::new(err_resp).render().unwrap());

        (status, body).into_response()
    }
}
