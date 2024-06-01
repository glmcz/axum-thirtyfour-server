
use std::fmt::format;

use axum::{
    //async_trait,
   // extract::{FromRef, FromRequestParts, State},
   // http::{request::Parts, StatusCode},
    extract::Query, response::{Html, IntoResponse}, routing::{get, post}, Router
};

mod footager;
mod admin;
// TODO upgrade to https://github.com/hyperium/tonic


#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(footager::user::footage_user_handler));
    let admin_app = Router::new().route("/admin", post(admin::admin::admin_handler));

    
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app.merge(admin_app)).await.unwrap();
}
