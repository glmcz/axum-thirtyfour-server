use tokio::signal;
use axum::{
  routing::{get, post}, Router
};

mod footager;
mod admin;
mod middleware;
// TODO upgrade to https://github.com/hyperium/tonic


#[tokio::main(flavor = "current_thread")]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", post(footager::user::footage_user_handler));
    let admin_app = Router::new().route("/admin", post(admin::admin::admin_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app.merge(admin_app))
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
