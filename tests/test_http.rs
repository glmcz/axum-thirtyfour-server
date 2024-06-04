#![allow(unused)]

use axum::Error as AxumError;
use eyre::Ok;
use eyre::Result;
use reqwest::Client;
use reqwest::header;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json::json;

// for runing tests with output, use
// cargo test -- --nocapture
// ********************************
#[tokio::test]
async fn footager_post() -> Result<()> {  // TODO FIX Error result...
    let post_footager = FootageUser {
        name: "user".into(),
        req_url: "https://domain.com".into(),
    };
    let footager_url = "http://localhost:3000/";
    _ = post(post_footager, footager_url).await;

        let post_admin = Admin {
        name: String::from("admin"),
        passwd: String::from("787458"),
    };
    let admin_url = "http://localhost:3000/admin";
    _ = post(post_admin, &admin_url).await;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Admin {
    name: String,
    passwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FootageUser {
    name: String,
    req_url: String,
}

async fn post<T: Serialize>(post: T, url: &str) -> Result<()> {  
    let json_payload = serde_json::to_string(&post).unwrap_or_else(|_| panic!("Failed to serialize"));
    println!("Sending JSON payload: {}", json_payload); 
    let resp = Client::new()
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .json(&post)
        .send()
        .await?;

    println!("Response status {}", resp.status());
    let text_response = &resp.text().await.unwrap_or_else(|_| panic!("Failed to read response body"));
    println!("Raw response body: {}", text_response);
    Ok(())
}
