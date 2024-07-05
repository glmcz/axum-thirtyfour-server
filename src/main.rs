use std::sync::Arc;
use tokio::signal;
use log::LevelFilter;
use axum::{
  routing::post, Router
};

mod footager;
mod admin;
mod middleware;
mod file_utils;
 mod webdriver;


use admin::logger::SimpleLogger;
// TODO upgrade to https://github.com/hyperium/tonic
use crate::webdriver::Selenium;


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), fantoccini::error::WebDriver> {

    // TODO make sure driver is arc, because it will be cloned for each incoming req.
    let driver = Selenium::init_selenium_driver("src/config.json").await?;


    _ = log::set_boxed_logger(Box::new(SimpleLogger))
    .map(|()| log::set_max_level(LevelFilter::max()));

    let app = Router::new()
    .route("/admin", post(admin::admin::admin_handler))
        .layer(axum::middleware::from_fn(admin::admin::admin_auth))

    .route("/footager", post(
        footager::user::footage_user_handler))
        //.layer(GlobalConcurrencyLimitLayer::new(3))  //Just for legacy, that this official layer don`t work without with_state(())
        //.with_state(());
        .layer(middleware::limiter::MyLayer::new(2))
        .with_state(Arc::new(driver.current_driver)); // there will be Queue for req in case of high load
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
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
