use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let app = Router::new().route("/", get(root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("Server started at 0.0.0.0:3000");
    axum::serve(listener, app).await?;

    Ok(())
}

#[tracing::instrument]
async fn root() -> &'static str {
    "Hello, World!!\n"
}
