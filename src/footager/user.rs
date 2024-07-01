use std::sync::Arc;
use std::time::Duration;
use axum::http::StatusCode;
use axum::{extract::Json, response::IntoResponse};
use axum::extract::State;

use serde::{Deserialize, Serialize};
use thirtyfour::WebDriver;
//use crate::footager::artgrid::run_artgrid_instance;

#[derive(Debug, Deserialize)]
pub struct FootageUserRequest {
    name: String,
    req_url: String,
}

#[derive(Serialize)]
pub enum FootageUserResponse {
    Ok,
    InternalError,
    BadRequest,
}




// fix get to post, because i told mate that i will wait for post req...
#[axum::debug_handler]
pub async fn footage_user_handler(State(state): State<Arc<WebDriver>>, Json(params): Json<FootageUserRequest>) -> impl IntoResponse {
    let name = params.name;
    let url = &params.req_url;
    // check inputs parameters
    println!("name {} url {}" , name, url.clone());

    tokio::time::sleep(Duration::from_secs(10)).await; // imitation of some long-running task
    // so far we are going to try 4 instances at once
    // let _ = run_artgrid_instance(state.clone(), url.clone()).await;
    // run_artgrid_instance(state.clone(), url.clone()).await?;
    // run_artgrid_instance(state.clone(), url.clone()).await?;
    // run_artgrid_instance(state.clone(), url.clone()).await?;


    (StatusCode::OK, "Request was received")
}

