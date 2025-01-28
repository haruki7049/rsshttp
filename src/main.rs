use axum::{routing::get, Router};
use clap::Parser;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = CLIArgs::parse();
    let address: String = format!("{}:{}", &args.ip, args.port);

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
    let listener = tokio::net::TcpListener::bind(&address).await?;

    // Running server
    tracing::info!("Server started at {}", &address);
    axum::serve(listener, app).await?;

    Ok(())
}

#[tracing::instrument]
async fn root() -> &'static str {
    tracing::debug!("Serving");
    "Hello, World!!\n"
}

#[derive(Parser)]
struct CLIArgs {
    /// Port number
    #[arg(short, long)]
    port: u32,

    /// Server's IP
    #[arg(short, long)]
    ip: String,
}
