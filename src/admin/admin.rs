use std::ops::Deref;
use axum::{body::Body, extract::Json, http::{header::CONTENT_TYPE, StatusCode}, response::{IntoResponse, Response}, extract::{Request, Extension}, RequestExt};
use axum::middleware::Next;
use axum::routing::head;
use tower::{Layer, ServiceExt};
use serde::{Deserialize, Serialize};

use std::task::{Context, Poll};
use std::time::Duration;
use tower::Service;
use rand::random;

#[derive(Deserialize, Serialize, Clone)]
pub struct Admin {
    //inner: S,
    name: String,
    passwd: String,
}

// impl <S, Request> Service<Request> for Admin<S>
//     where S:Service<Request>
// {
//  type Response = S::Response;
//     type Error = S::Error;
//     type Future = S::Future;
//
//     fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.inner.poll_ready(ctx)
//     }
//     fn call(&mut self, req: Request) -> Self::Future {
//         self.inner.call(req)
//     }
// }

#[derive(Serialize)]
pub struct AdminResponse {
    message: String,
}


pub async fn admin_handler(admin: Request) -> (StatusCode,Json<Admin>) {
    let admin: Json<Admin>  = admin.extract().await.unwrap();
    println!("req random{} and passwd {}", random::<i32>(), admin.passwd);
    tokio::time::sleep(Duration::from_secs(10)).await; // imitation of some long-running task
    (StatusCode::OK, admin)
}

// pub fn admin_authentication() -> impl Layer<axum::Router> {
//
// }

pub async fn admin_auth(mut req: Request, next: Next) -> Result<Response, StatusCode> /*http::Response<Body>*/ {
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth) = auth_header {
        auth
    } else {
        return Err(StatusCode::UNAUTHORIZED)
    };

    // if let Some(current_user) = authorize_current_user(auth_header).await {
        // insert the current user into a request extension so the handler can
        // extract it
        // req.extensions_mut().insert(current_user);

    Ok(next.run(req).await)
    // } else {
    //     Err(StatusCode::UNAUTHORIZED)
    // }
}

// Error handler layer
pub async fn error_handle(req: http::Request<Body>, next: Next) -> http::Response<Body> {
    let response = next.run(req).await;

    // Perform logic after calling the next service (optional)

    response
}