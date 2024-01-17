use std::net::SocketAddr;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use server::{
    routes::{authorization, decision, login, signup, stellar_info, verify},
    Handler,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

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

    let statics = Router::new()
        .route("/.well-known", get(|| async { todo!() }))
        .route("/hc", get(healthcheck));

    let clients = Router::new().route("/stellar", get(stellar_info)).route(
        "/authorize",
        get(authorization)
            .patch(decision::accept)
            .delete(decision::reject),
    );

    let accounts = Router::new()
        .route("/login", post(login))
        .route("/signup", post(signup))
        .route("/verify", post(verify));

    // Todo: Cors Setup
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    let app = Router::new()
        .nest("/", statics)
        .nest("/clients", clients)
        .nest("/accounts", accounts)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(handler);

    let bind = SocketAddr::from(([0, 0, 0, 0], 3854));
    let tcpl = tokio::net::TcpListener::bind(bind).await?;

    tracing::info!("Stellar Starting...");

    axum::serve(tcpl, app.into_make_service())
        .with_graceful_shutdown(exit())
        .await?;

    Ok(())
}

async fn healthcheck() -> impl IntoResponse {
    StatusCode::OK
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
