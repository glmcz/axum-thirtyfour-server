use axum::http::StatusCode;
use axum::{extract::Query, response::IntoResponse};
//use tokio::postgres::Client;
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
    let name = params.name.as_deref().unwrap_or("Default value");
    (StatusCode::OK, "Request was reciewed")
}

