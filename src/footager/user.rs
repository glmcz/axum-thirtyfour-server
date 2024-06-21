use std::time::Duration;
use axum::http::StatusCode;
use axum::{extract::Json, response::IntoResponse};

//use tokio::postgres::Client;
use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::middleware::bg_thread::BgController;
use crate::middleware::job::Task;

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
pub struct FootageUser {
    //pool: Client
}

impl FootageUser {
    pub fn parse_request(&self, req: FootageUserRequest) -> Result<()> {
        Ok(())
    }
}


// fix get to post, because i told mate that i will wait for post req...
#[axum::debug_handler]
pub async fn footage_user_handler(Json(params): Json<FootageUserRequest>) -> impl IntoResponse {
    let name = params.name;
    let url = &params.req_url;
    // check inputs parameters
    println!("name {} url {}" , name, url.clone());
    tokio::time::sleep(Duration::from_secs(10)).await; // imitation of some long-running task
    
    // create a new thread if not exists
    if let Some(mut instance) = BgController::get_instance(){
           
        if instance.has_no_job() {
            let job = Task::new("task".into(), url.clone(), "".into(), false);
            _ = instance.add_job(job);  
        }
        
    }else {
        // fist time use of bg thread...
        let mut bg = BgController::init();
        let job = Task::new("task".into(), url.clone(), "".into(), false);
        _ = bg.add_job(job);
        // create a new thread with job
    }


    (StatusCode::OK, "Request was reciewed")
}

