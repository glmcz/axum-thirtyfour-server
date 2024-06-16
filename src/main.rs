use axum::error_handling::{HandleError, HandleErrorLayer};
use reqwest::StatusCode;
use tokio::signal;
use std::sync::{Arc, RwLock};
use log::LevelFilter;
use axum::{
  routing::post, Router
};
use tower::{BoxError, ServiceBuilder};  // Never use ServiceBuilder if you don`t want to see Trait bounds hell

mod footager;
mod admin;
mod middleware;

use admin::logger::SimpleLogger;
// TODO upgrade to https://github.com/hyperium/tonic
use footager::limiter::MyLayer;


#[tokio::main(flavor = "current_thread")]
async fn main() {
    _ = log::set_boxed_logger(Box::new(SimpleLogger))
    .map(|()| log::set_max_level(LevelFilter::max()));

    let app = Router::new()
    .route("/admin", post(admin::admin::admin_handler))
        .layer(axum::middleware::from_fn(admin::admin::admin_auth))

    .route("/", post(
        footager::user::footage_user_handler))
        .layer(footager::limiter::MyLayer::new()); // there will be Queue for req in case of high load

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install CTRL_C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}