use crate::{data::Post, db::Handle, err::AppError, templ::TemplCommon};
use askama::Template;
use axum::{
    extract::{Extension, Path, Form},
    response::{Html, Redirect},
};
use color_eyre::eyre::eyre;
use serde::Deserialize;
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

#[derive(Deserialize)]
pub struct UpdateHandleForm {
    desc: String,
}

pub async fn post_update(
    hctx: Option<Handle>,
    Form(form): Form<UpdateHandleForm>,
    Path(uuid): Path<Uuid>,
    Extension(pool): Extension<db::Pool>,
) -> Result<Redirect, AppError> {
    if hctx.map(|h| h.id) != Some(uuid) {
        return Err(AppError::GenericISE(eyre!("trying to update the mypage for a handle which is not yours!")));
    }

    sqlx::query("UPDATE handle SET desc=? WHERE id=?")
        .bind(form.desc)
        .bind(uuid)
        .execute(&pool)
        .await?;

    Ok(Redirect::to(format!("/_/{}", uuid).parse().unwrap())) // TODO? can improve?
}