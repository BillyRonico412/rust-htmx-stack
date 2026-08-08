use anyhow::Ok;
use axum::Router;

const URL: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new();
    let listener = tokio::net::TcpListener::bind(URL).await?;
    println!("Server running to {}", URL);
    axum::serve(listener, app).await?;
    Ok(())
}
