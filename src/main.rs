use axum::error_handling::{HandleError, HandleErrorLayer};
use reqwest::StatusCode;
use tokio::signal;
use std::sync::{Arc, RwLock};
use log::LevelFilter;
use axum::{
  routing::post, Router
};
use tower::{BoxError, ServiceBuilder};

mod footager;
mod admin;
mod middleware;

use middleware::queue::Queue;
use admin::logger::SimpleLogger;
// TODO upgrade to https://github.com/hyperium/tonic

struct SharedState {
    queue: Queue,
}

type SharedAppState = Arc<SharedState>;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    _ = log::set_boxed_logger(Box::new(SimpleLogger))
    .map(|()| log::set_max_level(LevelFilter::max()));
    
    let shared_state = SharedAppState::new(SharedState { queue: Queue::new() });
    // build our application with a single route
    let app = Router::new().route("/", post(footager::user::footage_user_handler))
    // .layer(
    //     let a = ServiceBuilder::new();
    //     a.layer_fn(handle_error);
    //     a
    // )
    .route("/admin", post(admin::admin::admin_handler))
    .with_state(shared_state);
    
    // run our app with hyper, listening globally on port 3000
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


async fn handle_error(err: BoxError) -> (StatusCode, String) {
    (StatusCode::UNPROCESSABLE_ENTITY, err.to_string())
}