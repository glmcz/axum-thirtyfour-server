pub use fantoccini::{Client, Locator};
use fantoccini::actions::{InputSource, MouseActions, PointerAction,};
use fantoccini::elements::Element;
use fantoccini::error;
use serde_json::{Error, Value};
use tokio::time::Duration;

use crate::utils::{get_href_value, get_footage_id};

static ARTGRID_TEST_USER_INPUT: &str = "https://artgrid.io/clip/302105/boat-river-buildings-clouds";
static ARTGRID_SUBMIT_XPATH: &str = "//mat-dialog-container[@id=\'LoginDialog\']/art-login/div/div/div[2]/div/form/div[2]/art-spinner-button/div/button";
///html/body/div[5]/div[2]/div/mat-dialog-container                     /art-login/div/div/div[2]/div/form/div[2]/art-spinner-button/div/button
static ARTGRID_COLLECTION: &str = "//button[@class='art-user']";
static ARTGRID_SING_IN: &str = "https://artgrid.io/signin";
static ARTGRID_HOME_PAGE: &str = "https://artgrid.io";
static ARTGRID_CHECKOUT: &str = "/html/body/div[5]/div[2]/div/mat-dialog-container/art-my-cart/div/div/div[2]/button";


pub async fn artgrid_log_in(driver: &Client) -> Result<(), error::CmdError> {
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

pub async fn get_serialized_element_values(footage: Element) -> Result<Vec<Value>, error::CmdError> {
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

pub async fn click_on_hidden_download_button(client: &Client, args: Vec<serde_json::Value>) -> Result<(), error::CmdError> {
    // arguments[0]: An HTML element that is expected to be updated with new HTML content.
    // arguments[1]: A string containing HTML content that will be set as the inner HTML of the element.
    // arguments[2]: A callback function (done) that will be invoked after the HTML update is complete.
    // callback function return the element we want.
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
    if let Some(err) = client.execute_async(js_code, args).await.err() {
       eprintln!("{}", err); return Err(err)
    }else { Ok(()) }
}


// TODO allow download whole channel of author
// TODO every error throw down whole selenium script... is it really what we wont to? Maybe handle result instead of "?"
pub async fn run_artgrid_instance(client: &Client, user_url: &str) -> Result<String, error::CmdError> {
    client.goto(ARTGRID_HOME_PAGE).await?;
    client.maximize_window().await?;
    let login_element = client.find(Locator::Css("art-user")).await;

    if login_element.is_err()
    {
        artgrid_log_in(&client).await?;
    }

    //redirect to user footage
    client.goto(&user_url).await?;

    tokio::time::sleep(Duration::from_secs(2)).await;

    // there is some block from Artgrid side and this is workaround...
    // driver.find(Locator::Css("mat-list-text")).await?.click().await?;
    client.find(Locator::Id("main-logo-mobile")).await?.click().await?;
    client.back().await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    client.find(Locator::Css(".art-main-btn-t1.lg")).await?.click().await?;

    tokio::time::sleep(Duration::from_secs(2)).await;

    // pokud uz nechceme dal vybirat a davat do kosiku, tak hura do kosiku
    client.goto("https://artgrid.io/my-cart").await?;

    tokio::time::sleep(Duration::from_secs(2)).await;

    let proceed = client.find(Locator::XPath(ARTGRID_CHECKOUT)).await?;
    let mouse_actions = MouseActions::new("mouse".to_owned())
        .then(PointerAction::MoveToElement {
            element: proceed.clone(),
            duration: None,
            x: 1,
            y: 1,
        })
        .then(PointerAction::Pause {
            duration: Duration::from_secs(1)
        });

    client.perform_actions(mouse_actions).await?;
    proceed.click().await?;
    client.release_actions().await?;

    tokio::time::sleep(Duration::from_secs(2)).await;

    // // go to user collection
    client.find(Locator::XPath(ARTGRID_COLLECTION)).await?.click().await?;

    tokio::time::sleep(Duration::from_secs(2)).await;

    // use footage url to find footage in collection
    let footage = client
        .find(Locator::XPath(get_href_value(&user_url).as_str()))
        .await?;

    println!("{:#?}", footage.attr("href").await?);
    let footage = match get_serialized_element_values(footage).await {
        Ok(value) => value,
        Err(err) => return Err(err),
    };

    click_on_hidden_download_button(&client, footage).await?;
    // println!("Result: {:?}", res);

    // at the end extract user footage id for file identification on server side
    Ok(get_footage_id(&user_url))
}
