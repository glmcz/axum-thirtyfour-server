use std::rc::Rc;
use axum::http::StatusCode;
use axum::{extract::Json, response::IntoResponse};
use axum::extract::State;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tokio::sync::mpsc::Receiver;


#[derive(Debug, Deserialize)]
pub struct FootageUserRequest {
    name: String,
    url: String,
}

#[derive(Debug, Clone)]
pub struct FootageUser {
    recv: Rc<Receiver<String>>,
}

#[derive(Serialize)]
pub enum FootageUserResponse {
    Ok,
    InternalError,
    BadRequest,
}

#[derive(Debug)]
pub enum Command {
    Get {
        key: String,
        resp: Responder<String>,
    },
    // Set {
    //     id: u8,
    //     val: String,
    //     resp: Responder<String>,
    // },
}

type Responder<T> = oneshot::Sender<T>;

#[axum::debug_handler]
pub async fn footage_user_handler(State(state): State<Sender<Command>>, Json(params): Json<FootageUserRequest>) -> impl IntoResponse {
    let name = params.name;
    let url = &params.url;
    println!("name {} url {}" , name, url.clone());

    let (tx, rt) = oneshot::channel();
    let cmd = Command::Get { key: url.clone(),resp: tx };
    if let Err(e) = state.send(cmd).await{
        println!("{}", e);
    };

    let res = match rt.await{
        Ok(res) => {
                    println!("{}", res); Ok(res)
        },
        Err(e) => Err(e),
    };

    println!("result {:?}", res.err());
    println!("we received this downloaded url {}", url);
    (StatusCode::OK, "Request was received")
}


