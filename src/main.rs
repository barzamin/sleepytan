use async_sqlx_session::SqliteSessionStore;
use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service, post},
    AddExtensionLayer, Router,
};
use color_eyre::eyre::{Result, WrapErr};
use sqlx::sqlite::SqlitePool;
use std::{borrow::Cow, env};
use tower::{BoxError, ServiceBuilder};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod data;
mod db;
mod err;
mod handlers;
mod session;
mod templ;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let pool = SqlitePool::connect(
        &env::var("DATABASE_URL").wrap_err("no DATABASE_URL environment variable found")?,
    )
    .await?;

    let sess_store = SqliteSessionStore::from_client(pool.clone());
    sess_store.migrate().await?;

    let app = Router::new()
        .nest(
            "/static",
            get_service(ServeDir::new("static")).handle_error(|err: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("unhandled fs error: {}", err),
                )
            }),
        )
        .route("/", get(handlers::index::get))
        .nest(
            "/:code",
            Router::new()
                .route("/", get(handlers::board::get))
                .route("/post", post(handlers::thread::create)),
        )
        .route("/_/:id", get(handlers::thread::get))
        .route("/_/:id/reply", post(handlers::thread::reply))
        .route("/~:id", get(handlers::handle::get))
        .route("/~:id/update", post(handlers::handle::update))
        .route("/huh", get(handlers::misc::huh))
        .nest("/auth", handlers::auth::router())
        .layer(
            ServiceBuilder::new()
                .layer(CookieManagerLayer::new())
                .layer(AddExtensionLayer::new(pool))
                .layer(AddExtensionLayer::new(sess_store)),
        );

    let bind_addr = env::var("BIND_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string())
        .parse()
        .expect("couldn't parse bind address");
    axum::Server::bind(&bind_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
