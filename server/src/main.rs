use std::net::SocketAddr;

use axum::{Router, Server, routing::post};
use server::{InteractionHandler, routes::{signup_prepare, signup, verify}};
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let appender = tracing_appender::rolling::daily(std::path::Path::new("./logs/"), "debug.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()
            .with_filter(tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(|_| "driver=debug,server=debug".into())))
            .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG))
        .with(tracing_subscriber::fmt::Layer::default()
            .with_writer(non_blocking_appender)
            .with_ansi(false)
            .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG))
        .init();

    let handler = InteractionHandler::inject().await?;
    
    let app = Router::new()
        .route("/signup", post(signup_prepare))
        .route("/signup/:ticket", post(signup))
        .route("/verify/:ticket", post(verify))
        .with_state(handler);

    let bind = SocketAddr::from(([127, 0, 0, 1], 3854));

    Server::bind(&bind)
        .serve(app.into_make_service())
        .with_graceful_shutdown(exit())
        .await?;
    Ok(())
}

async fn exit() {
    let user_interrupt = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install keyboard interrupt.")
    };

    tokio::select! {
        _ = user_interrupt => {}
    }
}