use std::path::PathBuf;
use std::sync::Arc;

use axum::http::StatusCode;
use axum::{extract::Json, response::IntoResponse};
use axum::extract::State;
use serde::{Deserialize, Serialize};
use webscrapping::SeleniumOperations;
use tokio::task;
use crate::file_utils::file_helpers::file_downloaded;
use crate::footager::site_dispatch::run_site_instance;

#[derive(Debug, Deserialize)]
pub struct FootageUserRequest {
    name: String,
    pub url: String,
}

#[derive(Serialize)]
pub enum FootageUserResponse {
    Ok,
    InternalError,
    BadRequest,
}


#[derive(Clone)]
pub struct AppState<T: SeleniumOperations> {
    pub selenium: Arc<T>,
}


pub async fn footage_user_handler<T: SeleniumOperations>(State(state): State<AppState<T>>, Json(params): Json<FootageUserRequest>) -> impl IntoResponse {

    let res = task::spawn(async move {
        run_site_instance(state.selenium.get_driver(), params).await
    });

    match res.await {
        Ok(Ok(file_path)) => {
            let final_video_path = match task::spawn_blocking(move || {
                if !file_path.is_empty() {
                    file_downloaded(file_path, PathBuf::from("/Users/martindurak/Downloads"))
                } else {
                    Ok("sss".to_owned())
                }
            })
                .await
            {
                Ok(Ok(path)) => path,
                Ok(Err(e)) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            };

            println!("{:?} download path", final_video_path);
            (StatusCode::OK, "Request was received").into_response()
        }
        Ok(Err(e)) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}


