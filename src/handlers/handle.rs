use crate::{data::Post, db::Handle, err::AppError, templ::TemplCommon};
use askama::Template;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use uuid::Uuid;

use crate::db;

#[derive(Template)]
#[template(path = "handle.html")]
struct HandleTempl {
    handle: Option<Handle>,
    posts: Vec<Post>,
    common: TemplCommon,
}

pub async fn get(
    hctx: Option<Handle>,
    Path(uuid): Path<Uuid>,
    Extension(pool): Extension<db::Pool>,
) -> Result<Html<String>, AppError> {
    let handle = crate::db::get_handle(&pool, uuid).await.map_err(|err| {
        tracing::error!(%err, "failed to get handle");
        AppError::Db(err)
    })?;

    tracing::debug!(?handle, "fetched handle");

    let templ = HandleTempl {
        handle,
        posts: vec![],
        common: TemplCommon { hctx },
    };

    Ok(Html(templ.render().unwrap()))
}
