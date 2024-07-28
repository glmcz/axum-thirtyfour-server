#![allow(unused)]
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use dotenv::dotenv;
use fantoccini::Locator;

extern crate webscrapping;
use webscrapping::{utils, artgrid, Selenium, SeleniumOperations};

static ARTGRID_SUBMIT_XPATH: &str = "//mat-dialog-container[@id=\'LoginDialog\']/art-login/div/div/div[2]/div/form/div[2]/art-spinner-button/div/button";
static ARTGRID_SING_IN: &str = "https://artgrid.io/signin";
static ARTGRID_COLLECTION: &str = "//button[@class='art-user']";
// for runing tests with output, use
// cargo test -- --nocapture
// ********************************


/// Don't wait for full download.
#[tokio::test]
async fn test_user_footage_download() -> Result<(), fantoccini::error::CmdError> {
    let zk = dotenv().ok();
    let mut selenium = Selenium::new();
    let config_file = selenium.load_webriver_config_file("./../../webserver/src/config.json").unwrap();
    let _ = Selenium::start_selenium_server(config_file.geckodriver_path.as_str());

    let client = selenium.init_connection(config_file).await.unwrap();
    let _ = client.goto(ARTGRID_SING_IN).await.unwrap();

    if let Err(e) = artgrid::artgrid_log_in(&client).await{
        println!("{}", e);
    }

    // we should be inside user collection page
    client.find(Locator::XPath(ARTGRID_COLLECTION)).await?.click().await?;

    // TODO we can't choose footage which is not loaded on page
    let footage_lint = "https://artgrid.io/clip/373014/water-lily-pink-flower-flowering-plant-bloom";

    let footage = client
        .find(Locator::XPath(utils::get_href_value(footage_lint).as_str()))
        .await.unwrap();

    let args = artgrid::get_serialized_element_values(footage).await.unwrap();

    match artgrid::click_on_hidden_download_button(&client, args).await {
       Ok(_) => {}
        Err(e) => {
            println!("Error downloading: {}", e);
        }
    };

   client.close().await
}