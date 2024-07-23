use hyper::body::Incoming;
use hyper::service::HttpService;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server;
use tower::{Service, ServiceExt};
use tokio::sync::{mpsc, mpsc::Sender};
use tokio::{runtime, select, signal};
use tokio::net::TcpListener;
use log::LevelFilter;
use axum::{routing::post, Router, extract::Request, Error};
use dotenv::dotenv;
use fantoccini::Client;
use admin::logger::SimpleLogger;
use crate::file_utils::file_helpers::file_downloaded;

extern crate webscrapping;
use webscrapping::{Selenium};
use webscrapping::artgrid::run_artgrid_instance;
use footager::user::Command;

mod footager;
mod admin;
mod middleware;
mod file_utils;


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
    // Arc+ Mutex + clone need SEND trait for moving between threads. But Selenium doesn't have it...
    // let shared_recv = Arc::new(Mutex::new(main_receiver));

    threaded_rt.block_on( async {
        let axum_task = main_task(main_sender);

        let bg_task = threaded_rt.spawn(async {
            let driver = Selenium::init_selenium_driver("webserver/src/config.json").await.unwrap();
            // TODO handle missing conn to chromeDriver

            if let Some(e)  = second_task(main_receiver, driver.clone().current_driver).await.err() {
                eprintln!("bg_thread failed with err {}", e);
                // println!("Trying to recover: starting a new bg_thread");
                // need to impl Send for future...
                // or just use supervisor pattern and rewrite app
                // let _ = second_task(shared_recv.clone(), driver.clone().current_driver).await;
            }
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

async fn second_task(mut receiver: mpsc::Receiver<Command>, driver: Client) -> Result<(), Error> {
    //let mut recv = receiver.lock().unwrap();
    loop {
        select! {
            msg = receiver.recv() => {
                if let Some(cmd) = msg {
                    match cmd {
                        Command::Get { key, resp} => {
                            let file_path = match run_artgrid_instance(driver.clone(), key.as_str()).await {
                                // TODO upgrade file path String to std::path::Path
                                Ok(file_path) => file_path,
                                Err(e) => {
                                    println!("{}", e); return Err(Error::new(e))
                                }
                            };
                            let handle = tokio::spawn(async move {
                                let res = match file_downloaded(file_path).await {
                                    Ok(downloaded_file) => {
                                        if let Some(e) = resp.send(downloaded_file).err() {
                                            println!("{}", e);
                                        }
                                   },
                                   Err(e) => return Err(e),
                                };
                                Ok(res)
                            });
                            if let Err(e) = handle.await {
                                println!("{}", e);
                                return Err(Error::new(e));
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
