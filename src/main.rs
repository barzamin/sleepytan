use axum::{routing::get, AddExtensionLayer, Router};
use color_eyre::eyre::Result;
use sqlx::sqlite::SqlitePool;
use std::env;
use tower::ServiceBuilder;

mod data;
mod db;
mod err;
mod handlers;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let app = Router::new()
        .route("/:code", get(handlers::board::get))
        .route("/_/:id", get(handlers::handle::get))
        .layer(ServiceBuilder::new().layer(AddExtensionLayer::new(pool)));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
