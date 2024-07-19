#![allow(unused)]

use std::path::Path;
use std::process::Command;
use std::time::Duration;
use axum::Error as AxumError;
use dotenv::dotenv;
use eyre::Result;
use fantoccini::{ClientBuilder, Locator};
use fantoccini::elements::Element;
use fantoccini::error::{CmdError, ErrorStatus, WebDriver};
use fantoccini::error::ErrorStatus::UnknownError;
use fantoccini::wd::Capabilities;
use log::error;
use reqwest::Client;
use reqwest::header;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json::{Error, json, Value};

static ARTGRID_SUBMIT_XPATH: &str = "//mat-dialog-container[@id=\'LoginDialog\']/art-login/div/div/div[2]/div/form/div[2]/art-spinner-button/div/button";
static ARTGRID_SING_IN: &str = "https://artgrid.io/signin";
static ARTGRID_COLLECTION: &str = "//button[@class='art-user']";
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


async fn artgrid_log_in(driver: &fantoccini::Client) -> Result<(), fantoccini::error::CmdError> {
    driver.goto(ARTGRID_SING_IN).await?;
    tokio::time::sleep(Duration::from_secs(3)).await;

    let email_btn = driver.find(Locator::Id("mat-input-4")).await?;
    let passwd_btn = driver.find(Locator::Id("mat-input-5")).await?;

    let login = std::env::var("ARTGRID_EMAIL").expect("Can`t load .env variable for login");
    let passwd = std::env::var("ARTGRID_PASSWD").expect("Can`t load .env variable for password");

    email_btn.send_keys(login.as_str()).await?;
    passwd_btn.send_keys(passwd.as_str()).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    let submit = driver.find(Locator::XPath(ARTGRID_SUBMIT_XPATH)).await?;

    submit.click().await
}

async fn init_webdriver() -> fantoccini::Client {
    Command::new("pkill")
        .arg("chromedri")
        .spawn()
        .expect("failed");

    let path = Path::new("src/chromedriver");
    Command::new(path)
        .arg("--port=4444")
        .spawn()
        .expect("chrome driver is running");

    let cap: Capabilities = serde_json::from_str(
        r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--incognito"]}}"#,
    ).unwrap();
    //cap.insert("", "");
    let mut client = ClientBuilder::native();

   client.connect("http://localhost:4444").await.expect("localhost init failed")
}

async fn get_specific_footage_video(footage: Element) -> Result<Vec<Value>> {
    let res:Result<Value, Error> = match serde_json::to_value(footage.clone()) {
        Ok(value) => Ok(value),
        Err(err) => { println!("failed serialize footage {:?}", err);Err(err)}
    };

    let attr = match serde_json::to_value(footage.clone().attr("href").await.unwrap()){
        Ok(value) => Ok(value),
        Err(err) => { println!("failed serialize href  {:?}", err);Err(err)}
    };

    let args = vec![
        res.unwrap(),
        attr.unwrap(),
    ];

    Ok(args)
}

fn get_href_value(user_url: &str) -> String {
    if let Some(index) = user_url.find("/") {
        let href = format!(
            "{}{}{}",
            String::from("//a[@href='"),
            &user_url[index + 12..],
            "']"
        );
        href
    } else
    {
        "String is not a valid URL".to_string()
    }
    // Error: NoSuchElement("XPath(//a[@href='https://artgrid.io/clip/302105/boat-river-buildings-clouds'])")
}

/// Don't wait for full download.
#[tokio::test]
async fn test_user_footage_download() -> Result<(), fantoccini::error::CmdError> {
    dotenv().ok();

    let client = init_webdriver().await;
    let _ = client.goto(ARTGRID_SING_IN).await.unwrap();

    if let Err(e) = artgrid_log_in(&client).await{
        println!("{}", e);
    }

    // we should be inside user collection page
    client.find(Locator::XPath(ARTGRID_COLLECTION)).await?.click().await?;

    // TODO we can't choose footage which is not loaded on page
    let footage_lint = "https://artgrid.io/clip/373014/water-lily-pink-flower-flowering-plant-bloom";

    let footage = client
        .find(Locator::XPath(get_href_value(footage_lint).as_str()))
        .await.unwrap();

    let args = get_specific_footage_video(footage).await.unwrap();
    let js_code = r#"
        const hrefValue = arguments[1];
        let done = arguments[arguments.length - 1];

        // The target anchor selector might need adjustments
        const anchor = document.querySelector(`a[href*="${hrefValue}"]`);
        let hover;
        if (anchor) {
            // Trigger the download by hover and a click
            hover = anchor.parentNode.parentNode.parentNode;
            console.log(hover);
            hover.dispatchEvent(new MouseEvent('mouseenter', { 'bubbles': true }));
            let download = document.getElementsByClassName('art-download-to-action__link')[0];
            console.log("Hidden download button:", download);
            download.click();
            done(true); // Notify Selenium that the operation is successful
        } else {
            done(false); // Notify Selenium that the operation failed - anchor not found
        }
    "#;
   let res = client.execute_async(js_code, args).await.unwrap();
    // if no error download started
    // don't wait for full download.
   client.close().await
}
