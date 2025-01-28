use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // This section reads RUST_LOG env
    tracing_subscriber::fmt::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .json()
        .init();

    // Creating app & listener
    let app = Router::new().route("/", get(root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    // Running server
    tracing::info!("Server started at 0.0.0.0:3000");
    axum::serve(listener, app).await?;

    Ok(())
}

#[tracing::instrument]
async fn root() -> &'static str {
    tracing::debug!("Serving");
    "Hello, World!!\n"
}
