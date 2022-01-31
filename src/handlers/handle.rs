use crate::{data::Post, db::Handle, err::AppError};
use askama::Template;
use axum::{
    extract::{Extension, Path},
    response::Html,
};

use crate::db;

#[derive(Template)]
#[template(path = "handle.html")]
struct HandleTempl {
    handle: Option<Handle>,
    posts: Vec<Post>,
}

pub async fn get(
    Path(id): Path<i64>,
    Extension(pool): Extension<db::Pool>,
) -> Result<Html<String>, AppError> {
    let handle = crate::db::get_handle(&pool, id).await.map_err(|err| {
        tracing::error!(%err, "failed to get handle");
        AppError::Db(err)
    })?;

    tracing::debug!(?handle, "fetched handle");

    let templ = HandleTempl {
        handle,
        posts: vec![],
    };

    Ok(Html(templ.render().unwrap()))
}
