use crate::{data::Post, db::Handle, err::AppError, templ::TemplCommon};
use askama::Template;
use axum::{
    extract::{Extension, Form, Path},
    response::{Html, Redirect},
};
use chrono::{DateTime, Utc};
use color_eyre::eyre::eyre;
use serde::Deserialize;
use uuid::Uuid;

use crate::db;

struct FEPost {
    id: i64,
    create_ts: DateTime<Utc>,
    body: String,
    thread_subject: String,
    thread_id: i64,
}

#[derive(Template)]
#[template(path = "handle.html")]
struct HandleTempl {
    handle: Handle,
    postcount: i64,
    posts: Vec<FEPost>,
    common: TemplCommon,
}

#[derive(Template)]
#[template(path = "handle_404.html")]
struct Handle404Templ {
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

    if let Some(handle) = handle {
        tracing::debug!(?handle, "fetched handle");

        let (postcount,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM `post` WHERE handle = ?")
            .bind(handle.id)
            .fetch_one(&pool)
            .await?;

        let posts = sqlx::query_as!(FEPost, r#"SELECT post.id, post.body, post.create_ts as "create_ts: _", thread.subject as thread_subject, thread.id as thread_id
FROM post
INNER JOIN thread ON
    thread.id = post.thread
WHERE handle = ?"#, handle.id)
            .fetch_all(&pool)
            .await?;

        let templ = HandleTempl {
            handle,
            postcount,
            posts,
            common: TemplCommon { hctx },
        };

        Ok(Html(templ.render().unwrap()))
    } else {
        Ok(Html(
            Handle404Templ {
                common: TemplCommon { hctx },
            }
            .render()
            .unwrap(),
        ))
    }
}

#[derive(Deserialize)]
pub struct UpdateHandleForm {
    desc: String,
}

pub async fn update(
    hctx: Option<Handle>,
    Form(form): Form<UpdateHandleForm>,
    Path(uuid): Path<Uuid>,
    Extension(pool): Extension<db::Pool>,
) -> Result<Redirect, AppError> {
    if hctx.map(|h| h.id) != Some(uuid) {
        return Err(AppError::GenericISE(eyre!(
            "trying to update the mypage for a handle which is not yours!"
        )));
    }

    sqlx::query("UPDATE handle SET desc=? WHERE id=?")
        .bind(form.desc)
        .bind(uuid)
        .execute(&pool)
        .await?;

    Ok(Redirect::to(format!("/~{}", uuid).parse().unwrap())) // TODO? can improve?
}
