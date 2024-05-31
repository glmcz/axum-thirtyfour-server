use axum::response::Html;
use axum::{extract::Query, response::IntoResponse};
//use tokio::postgres::Client;
use axum::{extract::Extension};
use eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct FootageUserRequest {
    name: Option<String>,
    //req_url: String,
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

pub async fn footage_user_handler(Query(params): Query<FootageUserRequest>) -> impl IntoResponse {
    println!("Handler info -");
    let name = params.name.as_deref().unwrap_or("Default value");
    Html(format!("Hello {name}"))
}

