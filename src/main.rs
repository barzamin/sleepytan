use async_session::{async_trait, MemoryStore};
use axum::{
    extract::{FromRequest, RequestParts},
    response::{Html, IntoResponse, Response},
    routing::get,
    AddExtensionLayer, Router,
};
use color_eyre::eyre::Result;
use sqlx::sqlite::SqlitePool;
use std::env;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;

mod data;
mod db;
mod err;
mod handlers;
mod session;

use session::AccessorCtx;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let sess_store = MemoryStore::new();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let app = Router::new()
        .route("/", get(handlers::index::get))
        .route("/:code/", get(handlers::board::get))
        .route("/_/:id", get(handlers::handle::get))
        .nest("/auth", handlers::auth::router())
        .layer(
            ServiceBuilder::new()
                .layer(CookieManagerLayer::new())
                .layer(AddExtensionLayer::new(pool))
                .layer(AddExtensionLayer::new(sess_store)),
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
