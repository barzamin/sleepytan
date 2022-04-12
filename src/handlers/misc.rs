use askama::Template;
use axum::response::Html;

use crate::{db::Handle, err::AppError, templ::TemplCommon};

#[derive(Template)]
#[template(path = "huh.html")]
struct HuhTempl {
    common: TemplCommon,
}

pub async fn huh(
    hctx: Option<Handle>,
) -> Result<Html<String>, AppError> {
    let templ = HuhTempl {
        common: TemplCommon { hctx },
    };

    Ok(Html(templ.render().unwrap()))
}
