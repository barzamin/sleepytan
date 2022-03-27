use argon2::{Argon2, PasswordHash, PasswordVerifier};
use askama::Template;
use async_session::{MemoryStore, Session, SessionStore};
use async_sqlx_session::SqliteSessionStore;
use axum::{
    extract::{Extension, Form},
    http::{StatusCode, Uri},
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Router,
};
use color_eyre::eyre::eyre;
use password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use tracing::{debug, info};
use uuid::Uuid;

use crate::db;
use crate::{err::AppError, session::SESSION_COOKIE_NAME};

#[derive(Template)]
#[template(path = "signup.html")]
struct SignupTempl {}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTempl {}

pub fn router() -> Router {
    Router::new()
        .route("/signup", get(get_signup).post(post_signup))
        .route("/login", get(get_login).post(post_login))
}

async fn get_signup() -> Html<String> {
    let templ = SignupTempl {};

    Html(templ.render().unwrap())
}

#[derive(Deserialize)]
struct SignupForm {
    name: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginForm {
    id: Uuid,
    password: String,
}

async fn get_login() -> Html<String> {
    let templ = LoginTempl {};

    Html(templ.render().unwrap())
}

async fn post_login(
    Form(form): Form<LoginForm>,
    Extension(pool): Extension<db::Pool>,
    Extension(sess_store): Extension<SqliteSessionStore>,
    cookies: Cookies,
) -> Result<impl IntoResponse, AppError> {
    let handle = crate::db::get_handle(&pool, form.id).await?.unwrap(); // FIXME(3moon)!!
    let hash = PasswordHash::new(&handle.passhash)?;

    let verif = Argon2::default().verify_password(form.password.as_bytes(), &hash);

    debug!(?verif, "verification of passhash");
    if let Ok(_) = verif {
        // insert session
        debug!("good login; inserting session and redirecting to /");

        let mut sess = Session::new();
        sess.insert("id", handle.id)
            .map_err(|_| eyre!("can't insert id to session"))?;
        let cookieval = sess_store.store_session(sess).await?.unwrap();
        cookies.add(crate::session::new_cookie(cookieval));

        Ok(Redirect::to(Uri::from_static("/")).into_response())
    } else {
        let templ = LoginTempl {};
        Ok((StatusCode::UNAUTHORIZED, Html(templ.render().unwrap())).into_response())
    }
}

async fn post_signup(
    form: Form<SignupForm>,
    Extension(pool): Extension<db::Pool>,
    Extension(sess_store): Extension<SqliteSessionStore>,
    cookies: Cookies,
) -> Result<impl IntoResponse, AppError> {
    let formdata = form.0;

    let salt = SaltString::generate(&mut OsRng);
    let pwhash = Argon2::default().hash_password(formdata.password.as_bytes(), &salt)?;

    let id = crate::db::insert_handle(&pool, &formdata.name, &pwhash).await?;
    info!(%id, "registered new handle");

    let mut sess = Session::new();
    sess.insert("id", id)
        .map_err(|_| eyre!("can't insert id to session"))?;
    let cookieval = sess_store.store_session(sess).await?.unwrap();
    cookies.add(crate::session::new_cookie(cookieval));

    Ok(Redirect::to(Uri::from_static("/")))
}
