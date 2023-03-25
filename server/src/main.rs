use std::net::SocketAddr;

use axum::{Router, Server, routing::post};
use server::{InteractionHandler, routes::{signup_prepare, signup, verify}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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