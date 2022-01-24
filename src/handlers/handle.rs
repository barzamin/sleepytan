use crate::{data::Post, err::AppError};
use askama::Template;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use sqlx::SqlitePool;

#[derive(Template)]
#[template(path = "handle.html")]
struct HandleTempl {
    handle: String,
    posts: Vec<Post>,
}

pub async fn get(
    Path(id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Html<String>, AppError> {
    let handle = crate::db::get_handle(&pool, id).await.map_err(|err| {
        tracing::error!(%err, "failed to get handle");
        AppError::Db(err)
    })?;

    tracing::debug!(?handle, "fetched handle");

    let templ = HandleTempl {
        handle: "3moon".to_string(),
        posts: vec![],
    };

    Ok(Html(templ.render().unwrap()))
}
