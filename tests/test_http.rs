#![allow(unused)]

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
async fn default_get() -> Result<()> {
    let resp = Client::new()
        .get("http://localhost:3000/")
        .header(CONTENT_TYPE, "application/json")
        .json("get request")
        .send()
        .await
        .expect("not failture from get");
     

    // Pretty print the result (status, headers, response cookies, client cookies, body)
    println!("Post response status {:?} and text {:?}", resp.status(), resp.text().await?);
    Ok(())
}

#[tokio::test]
async fn default_post() -> Result<()> {

    #[derive(Debug, Serialize, Deserialize)]
    struct PostExample {
        name: String,
        passwd: String,
    };

    let post_example = PostExample {
        name: String::from("admin"),
        passwd: String::from("787458"),
    };

    let resp = Client::new()
        .post("http://localhost:3000/admin")
        .header(CONTENT_TYPE, "application/json")
        .json(&post_example)
        .send()
        .await?;

    let status = resp.status();
    // Handle the response body more gracefully
    if status.is_success() {
        println!("Post response is {:?}", resp.json::<PostExample>().await?);
    } else {
        eprintln!("Failed to post: {}", resp.status());
    }
    Ok(())
}
