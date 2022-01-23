use axum::{routing::get, Router, AddExtensionLayer};
use sqlx::sqlite::SqlitePool;
use color_eyre::eyre::Result;
use tower::ServiceBuilder;
use std::env;

mod handlers;
mod data;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let app = Router::new()
        .route("/", get(handlers::board::board))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(pool))
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
