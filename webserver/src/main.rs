use std::sync::Arc;
use hyper::body::Incoming;
use hyper::service::HttpService;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server;
use tower::{Service, ServiceExt};
use tokio::{runtime, signal};
use tokio::net::TcpListener;
use log::LevelFilter;
use axum::{routing::post, Router, extract::Request};
use dotenv::dotenv;
use admin::logger::SimpleLogger;

extern crate webscrapping;
use webscrapping::{SeleniumOperations, Selenium};
use crate::footager::user::AppState;

mod footager;
mod admin;
mod middleware;
mod file_utils;

// TODO upgrade to https://github.com/hyperium/tonic
fn router_setup(sender: AppState<Selenium>) -> Router<()> {
    Router::new()
        .route("/admin", post(admin::admin::admin_handler))
        .layer(axum::middleware::from_fn(admin::admin::admin_auth))
        .route("/footager", post( footager::user::footage_user_handler))
        .layer(middleware::limiter::MyLayer::new(2)) // there add sender
        .with_state(sender)
}

//#[tokio::main(worker_threads = 2)]
fn main() -> Result<(), fantoccini::error::WebDriver> {
    dotenv().ok();
    _ = log::set_boxed_logger(Box::new(SimpleLogger))
        .map(|()| log::set_max_level(LevelFilter::max()));

   let threaded_rt = runtime::Builder::new_multi_thread()
       .worker_threads(2)
       .enable_all()
       .build()
       .unwrap();

    // TODO tokio CancellationToken
    threaded_rt.block_on( async {
        let mut selenium = Selenium::new();
        let config_file = selenium.load_webriver_config_file("webserver/src/config.json").unwrap_or_default();
        //TODO refactor main and handle error
        let _ = Selenium::start_selenium_server(config_file.geckodriver_path.as_str());
        let _ =  selenium.init_connection(config_file).await.unwrap();

        let axum_task = main_task(AppState{selenium: Arc::new(selenium)});
        tokio::join!(axum_task);
    });

    Ok(())
}

async fn main_task(state: AppState<Selenium>) {
    let listener = TcpListener::bind("localhost:3000").await.unwrap();
    loop {
        let (socket, _remote_addr) = listener.accept().await.unwrap();
        let middleware_service = router_setup(state.clone());

        tokio::spawn(async move {
            let socket = TokioIo::new(socket);
            let hyper_service = hyper::service::service_fn(move |req: Request<Incoming>| {
                middleware_service.clone().call(req)
            });

            if let Err(err) = server::conn::auto::Builder::new(TokioExecutor::new())
                .serve_connection(socket, hyper_service)
                .await
            {
                eprintln!("failed to server connection: {err:#}");
            }
        });
    }
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
