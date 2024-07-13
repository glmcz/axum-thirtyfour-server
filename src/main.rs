use hyper::body::Incoming;
use hyper::service::HttpService;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server;
use tower::{Service, ServiceExt};
use tokio::sync::{mpsc, mpsc::Sender};
use tokio::{runtime, select, signal};
use tokio::net::TcpListener;
use log::LevelFilter;
use axum::{
  routing::post, Router,
  extract::Request,
};
use dotenv::dotenv;
use fantoccini::Client;
use admin::logger::SimpleLogger;
use crate::footager::artgrid::run_artgrid_instance;
use crate::footager::user::Command;
use crate::webdriver::Selenium;

mod footager;
mod admin;
mod middleware;
mod file_utils;
 mod webdriver;

// TODO upgrade to https://github.com/hyperium/tonic
fn router_setup(sender: Sender<Command>) -> Router<()> {
    Router::new()
        .route("/admin", post(admin::admin::admin_handler))
        .layer(axum::middleware::from_fn(admin::admin::admin_auth))
        .route("/footager", post( footager::user::footage_user_handler))
        .layer(middleware::limiter::MyLayer::new(2)) // there add sender
        .with_state(sender.clone())
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

    // we need bi-directional communication between 2 threads. So far crossbeam is the fastest option
    // Update1 we need to switch into tokio oneShot async for not to block handle function. Crossbeam is blocking by default
    // but we are loosing queue... which we can use in limiter and if service will be free send it to bg_thread for processing
    // Update2 we can`t use oneShot because it doesn`t have Clone and it is designed for SPSC,
    // but we are adding it into handler for every req so we need MPMC...
    let (main_sender, mut main_receiver) = tokio::sync::mpsc::channel(100);

    threaded_rt.block_on( async {
        let axum_task = threaded_rt.spawn(async {
            main_task(main_sender).await;
        });

        let bg_task = threaded_rt.spawn(async {
            let driver = Selenium::init_selenium_driver("src/config.json").await.unwrap();
            // TODO handle missing conn to chromeDriver
            second_task(main_receiver, driver.current_driver).await;
        });
        tokio::join!(axum_task, bg_task);
    });

    Ok(())
}

async fn main_task(main_sender: Sender<Command>) {
    let listener = TcpListener::bind("localhost:3000").await.unwrap();
    loop {
        let (socket, _remote_addr) = listener.accept().await.unwrap();
        let middleware_service = router_setup(main_sender.clone());

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

async fn second_task(mut recv: mpsc::Receiver<Command>, driver: Client){
    loop {
        select! {
            msg = recv.recv() => {
                if let Some(cmd) = msg {
                    match cmd {
                        Command::Get { key, resp} => {
                            let res = match run_artgrid_instance(driver.clone(), key.as_str()).await{
                                Ok(url) => Ok(url),
                                Err(e) => {
                                    println!("{}", e);
                                    Err(e)
                                }
                            };

                            if let Some(e) = resp.send(res.unwrap().to_string()).err(){
                                println!("{}", e);
                            }
                        }
                    }
                }
            }
            //task relax
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
            }
        }
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
