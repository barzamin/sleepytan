use async_session::{async_trait, SessionStore};
use async_sqlx_session::SqliteSessionStore;
use axum::{
    extract::{Extension, FromRequest, RequestParts},
    http::Uri,
    response::{IntoResponse, Redirect, Response},
};
use color_eyre::eyre::eyre;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;
use uuid::Uuid;

use crate::{
    db::{self, Handle},
    err::AppError,
};

pub const SESSION_COOKIE_NAME: &str = "aaasession";

pub fn new_cookie(cookieval: String) -> Cookie<'static> {
    Cookie::build(SESSION_COOKIE_NAME, cookieval)
        .path("/")
        .finish()
}

#[async_trait]
impl<B> FromRequest<B> for Handle
where
    B: Send,
{
    type Rejection = Response;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let redirect = || Redirect::to(Uri::from_static("/auth/login")).into_response();

        let extensions = req.extensions().ok_or(
            AppError::GenericISE(eyre!(
                "missing extensions: have they been taken by another extractor?"
            ))
            .into_response(),
        )?;

        let cookies = extensions
            .get::<Cookies>()
            .cloned()
            .ok_or(AppError::GenericISE(eyre!("missing Cookies extension!")).into_response())?;

        let session_store = Extension::<SqliteSessionStore>::from_request(req)
            .await
            .map_err(|_| {
                AppError::GenericISE(eyre!("missing session store extension!")).into_response()
            })?;

        let db = Extension::<db::Pool>::from_request(req)
            .await
            .map_err(|_| AppError::GenericISE(eyre!("missing db extension!")).into_response())?;

        let sesscookie = cookies.get(SESSION_COOKIE_NAME);
        debug!(?sesscookie, "got session cookie");
        let sesscookie = sesscookie.ok_or_else(redirect)?;

        let session = session_store
            .load_session(sesscookie.value().to_owned())
            .await
            .map_err(|err| AppError::Session(err).into_response())?
            .ok_or_else(redirect)?;

        let id: Option<Uuid> = session.get("id");
        let id = id.ok_or_else(redirect)?;

        debug!(%id, "loaded session with handle id");

        let handle = crate::db::get_handle(&db, id).await.map_err(|err| {
            tracing::error!(%err, "failed to get handle");
            AppError::Db(err).into_response()
        })?;

        handle.ok_or_else(|| {
            AppError::GenericISE(eyre!(
                "no handle in database matching session id: user deleted but session remains?"
            ))
            .into_response()
        })
    }
}
