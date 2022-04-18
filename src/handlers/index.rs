use askama::Template;
use axum::{extract::Extension, response::Html};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::data::Board;
use crate::db::{self, Handle};
use crate::err::AppError;
use crate::templ::TemplCommon;

struct FEPost {
    id: i64,
    body: String,
    create_ts: DateTime<Utc>,
    handle_name: String,
    handle_id: Uuid,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTempl {
    common: TemplCommon,
    boards: Vec<Board>,
    recent_posts: Vec<FEPost>,
}

pub async fn get(
    hctx: Option<Handle>,
    Extension(pool): Extension<db::Pool>,
) -> Result<Html<String>, AppError> {
    let boards = db::get_boards(&pool).await?;

    let recent_posts = sqlx::query_as!(FEPost, r#"
    SELECT post.id, post.body, post.create_ts as "create_ts: _", handle.name as handle_name, handle.id as "handle_id!: _"
    FROM post
    INNER JOIN handle ON
        handle.id = post.handle
    ORDER BY post.create_ts DESC
    LIMIT ?
    "#, 10i64).fetch_all(&pool).await?;

    let templ = IndexTempl {
        boards,
        recent_posts,
        common: TemplCommon { hctx },
    };

    Ok(Html(templ.render().unwrap()))
}
