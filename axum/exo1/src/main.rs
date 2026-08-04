use axum::{Router, response::Html, routing::get};
use std::format;

const PORT: u32 = 3000;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/", get(|| async { Html("Hello <strong>World</strong>") }));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT)).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
