use std::net::SocketAddr;

use axum::{Router, Server, routing::{post, get}, response::IntoResponse, http::StatusCode};
use server::{Handler, routes::{verify, signup, stellar_info, authorization}};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let appender = tracing_appender::rolling::daily(std::path::Path::new("./logs/"), "debug.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()
            .with_filter(tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "driver=debug,server=debug,tower_http=debug,hyper=debug,lettre=debug,sqlx=debug".into())))
            .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG))
        .with(tracing_subscriber::fmt::Layer::default()
            .with_writer(non_blocking_appender)
            .with_ansi(false)
            .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG))
        .init();

    let handler = Handler::init().await?;

    let clients = Router::new()
        .route("/authorize", get(authorization));

    let app = Router::new()
        .route("/.well-known", get(|| async { todo!() })) // Todo: To produce endpoints so that JWK public keys can be provided.
        .route("/healthcheck", get(healthcheck))
        .route("/signup", post(signup))
        .route("/verify", post(verify))
        .route("/info", get(stellar_info))
        .nest("/clients", clients)
        .layer(TraceLayer::new_for_http())
        .with_state(handler);

    let bind = SocketAddr::from(([0, 0, 0, 0], 3854));

    tracing::info!("Stellar Starting...");

    Server::bind(&bind)
        .serve(app.into_make_service())
        .with_graceful_shutdown(exit())
        .await?;

    Ok(())
}

async fn healthcheck() -> impl IntoResponse {
    StatusCode::ACCEPTED
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