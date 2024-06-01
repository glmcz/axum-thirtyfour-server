
use axum::{
    body::Body,
    extract::Json, http::{header::CONTENT_TYPE, StatusCode}, 
    response::{IntoResponse, Response}, 
};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct Admin {
    name: String,
    passwd: String,
}

#[derive(Serialize)]
pub struct AdminResponse {
    message: String,
}


pub async fn admin_handler(Json(mut admin): Json<Admin>) -> Json<Admin> {
  
    // let response = AdminResponse {
    //     message: "Admin data received successfully".into(),
    // };

    admin.name = "different".into();
    //let serialized_resp = json!(response);
    //let body = serde_json::to_string(&response).unwrap();

    Json(admin)
   // (StatusCode::OK, Json(admin))
}