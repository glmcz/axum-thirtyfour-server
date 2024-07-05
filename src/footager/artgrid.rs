use std::any::Any;
use std::mem::zeroed;
use std::sync::Arc;
use file_helpers::file_downloaded;

use tokio::time::Duration;
use crate::file_utils::file_helpers;

static ARTGRID_TEST_USER_INPUT: &str = "https://artgrid.io/clip/302105/boat-river-buildings-clouds";
static ARTGRID_SUBMIT_XPATH: &str = "//mat-dialog-container[@id=\'LoginDialog\']/art-login/div/div/div[2]/div/form/div[2]/art-spinner-button/div/button";
///html/body/div[5]/div[2]/div/mat-dialog-container                     /art-login/div/div/div[2]/div/form/div[2]/art-spinner-button/div/button
static ARTGRID_EMAIL: &str = "mates.glm@gmail.com";
static ARTGRID_PASSWORD: &str = "PosledniNadeje1";
static ARTGRID_COLLECTION: &str = "//button[@class='art-user']";
static ARTGRID_SING_IN: &str = "https://artgrid.io/signin";
static ARTGRID_HOME_PAGE: &str = "https://artgrid.io";
static ARTGRID_CHECKOUT: &str = "/html/body/div[5]/div[2]/div/mat-dialog-container/art-my-cart/div/div/div[2]/button";


use fantoccini::{Client, Locator};
use fantoccini::actions::{InputSource, MOUSE_BUTTON_MIDDLE, MouseActions, PointerAction, WheelAction};
use fantoccini::error::{CmdError, ErrorStatus};
use serde_json::{Error, Value};

fn get_href_value(user_url: &String) -> String {
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

fn get_footage_id(user_url: &String) -> String {
    if let Some(index) = user_url.find("clip/") {
        let href = format!("{}", &user_url[index + 5..]);
        println!("{}", index.to_string());
        if let Some(posfix) = href.find("/") {
            let end = format!("{}", &href[..posfix]);
            end
        } else {
            "Invalid URL: No slash after 'clip/'".to_string()
        }
    } else {
        "String is not a valid URL".to_string()
    }
}

// TODO proper error
pub async fn run_artgrid_instance(driver: Arc<Client>, user_url: String) -> Result<String, fantoccini::error::CmdError> {

    driver.goto(ARTGRID_HOME_PAGE).await?;
    driver.maximize_window().await?;
    let login_element = driver.find(Locator::Css("art-user")).await;

    if login_element.is_err()
    {
        driver.goto(ARTGRID_SING_IN).await?;
        tokio::time::sleep(Duration::from_secs(3)).await;

        let email = driver.find(Locator::Id("mat-input-4")).await?;
        let pass = driver.find(Locator::Id("mat-input-5")).await?;

        // TODO allow download whole channel of author

        email.send_keys(ARTGRID_EMAIL).await?;
        pass.send_keys(ARTGRID_PASSWORD).await?;
        tokio::time::sleep(Duration::from_secs(2)).await;

        let submit = driver.find(Locator::XPath(ARTGRID_SUBMIT_XPATH)).await?;
        submit.click().await?;

        // // Obsolate save all cookies into file so we wouldn't have to authenticate again
        // let cookies = driver.get_all_cookies().await?;
        // cookies_write_to_file(cookies, ChooseDomain::Artgrid);

        // Wait until the submit button no longer exists
        // submit.wait_until().condition(f)
        tokio::time::sleep(Duration::from_secs(4)).await;
    }

    //redirect to user footage
    let res = driver.goto(&user_url).await;

    tokio::time::sleep(Duration::from_secs(1)).await;
    //add item to card

    // there is some block from Artgrid side and this is workaround...
    driver.find(Locator::Id("main-logo-mobile")).await?.click().await?;
    driver.back().await?;
    tokio::time::sleep(Duration::from_secs(1)).await;

    driver.find(Locator::Css(".art-main-btn-t1.lg")).await?.click().await?;

    tokio::time::sleep(Duration::from_secs(2)).await;

    // pokud uz nechceme dal vybirat a davat do kosiku, tak hura do kosiku
    driver.goto("https://artgrid.io/my-cart").await?;

    // checkout kosik
    tokio::time::sleep(Duration::from_secs(2)).await;
    //TODO need to disable it because it block find method which we run after, so now workaround with sleep is used

    let proceed = driver.find(Locator::XPath(ARTGRID_CHECKOUT)).await?;
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

    driver.perform_actions(mouse_actions).await?;
    proceed.click().await?;
    driver.release_actions().await?;

    tokio::time::sleep(Duration::from_secs(2)).await;

    // // go to user collection
    driver.find(Locator::XPath(ARTGRID_COLLECTION)).await?.click().await?;

    tokio::time::sleep(Duration::from_secs(2)).await;

    // use footage url to find footage in collection
    let footage = driver
        .find(Locator::XPath(get_href_value(&user_url).as_str()))
        .await?;

    println!("{:#?}", footage.attr("href").await?);
    let res = match serde_json::to_value(footage.clone()) {
        Ok(value) => Ok(value),
        Err(err) => { println!("failed serialize footage {:?}", err);Err(err)}
    };

    let attr = match serde_json::to_value(footage.clone().attr("href").await?){
        Ok(value) => Ok(value),
        Err(err) => { println!("failed serialize footage {:?}", err);Err(err)}
    };

    let args = vec![
        res.unwrap(),
        attr.unwrap(),
    ];
    // arguments[0]: An HTML element that is expected to be updated with new HTML content.
    // arguments[1]: A string containing HTML content that will be set as the inner HTML of the element.
    // arguments[2]: A callback function (done) that will be invoked after the HTML update is complete.
    //callback function return the element we want to

    // adjusting timeout for really long downloading requests
    // driver.set_script_timeout(std::time::Duration::from_secs(120)).await?;

    let js_code = r#"
        const hrefValue = arguments[1];
        const done = arguments[arguments.length - 1];

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
    //Result: ScriptRet { handle: SessionHandle { session_id: "da64ed23-66b1-4434-8541-368474193d2a" }, value: Bool(false) }
    let ret = match driver.execute(js_code, args).await{
        Ok(res) => Ok(res),
        Err(err) => { println!("JS error {:?}", err);Err(err)}
    };
    println!("Result: {:?}", ret);


    //TODO if file is in download folder twice (1), (2), we need to choose newest one and checking if
    //TODO resume, error, the file`s size is growing or not. Size can`t be 0;

    //in the end extract user footage id for file identification on file server side
    let id = get_footage_id(&user_url);

    //file download check returning path of downloaded file
    match file_downloaded(id){
        Ok(downloaded_file) => Ok(downloaded_file),
        Err(err) => Err(CmdError::Lost(err.into())),
    }

    // TODO every error throw down whole selenium script...
    // is it realy what we wont to?
}