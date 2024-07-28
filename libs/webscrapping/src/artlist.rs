use fantoccini::{Client, error, Locator};
use tokio::time::Duration;
use crate::utils::get_footage_name;

static ARTLIST_LOGIN_SUBMIT: &str = "//button[@type=\'submit\']";
static ARTLIST_LOGIN_EMAIL_INPUT: &str = "//input[@type=\'email\']";
static ARTLIST_LOGIN_PASSWORD_INPUT: &str = "//input[@type=\'password\']";
static ARTLIST_DOWNLOAD_BTN: &str = "//div[@id=\'song-page-react\']/div/div[3]/div/div/div/div/div/div/div/div/button";
static ARTLIST_DOWNLOAD_WAV: &str = "//button[contains(.,\'WAV\')]";
static ARTLIST_LOGIN_BTN: &str = "//div[3]/div/div/div";
static ARTLIST_HOME_PAGE: &str = "https://artlist.io";

/// static TEST_INPUTS: &str = "https://artlist.io/royalty-free-music/song/morning-has-come/127054";
///                             https://artlist.io/sfx/track/bora-wind---forest-strong-wind-dead-leaves-rustling/54298
///                             https://artlist.io/sfx/track/experimental-whooshes--transitions---spaceship-metallic-whoosh-/124747
///                             https://artlist.io/royalty-free-music/song/kimray/92445
///                             https://artlist.io/royalty-free-music/song/hero/114919

async fn log_in_artlist(client: &Client) -> Result<(), error::CmdError> {
    // driver.goto(ARTLIST_HOME_PAGE).await?;
    // tokio::time::sleep(Duration::from_secs(1)).await;
    //singin button to trigger login form
    let login = client.find(Locator::XPath(ARTLIST_LOGIN_BTN)).await?;
    login.click().await?;

    tokio::time::sleep(Duration::from_secs(2)).await;
    let email = client.find(Locator::XPath(ARTLIST_LOGIN_EMAIL_INPUT)).await?;
    let pass = client.find(Locator::XPath(ARTLIST_LOGIN_PASSWORD_INPUT)).await?;

    email.send_keys(std::env::var("ARTLIST_EMAIL").expect("login env should be loaded").as_str()).await?;
    pass.send_keys(std::env::var("ARTLIST_PASSWORD").expect("password env should be loaded").as_str()).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    let submit = client.find(Locator::XPath(ARTLIST_LOGIN_SUBMIT)).await?;
    submit.click().await?;

    tokio::time::sleep(Duration::from_secs(2)).await;
    Ok(())

}

pub async fn run_artlist_instance(client: &Client, user_url: &str) -> Result<String, error::CmdError> {
    client.goto(ARTLIST_HOME_PAGE).await?;
    client.maximize_window().await?;

    ///Xpath: html/body/div[9]/div/div/div/div/div/div[3]/button/span/span
    let dialog_popup = client.find(Locator::Css(".flex.justify-center.items-center.disabled\\:cursor-not-allowed.disabled\\:text-primary.disabled\\:bg-gray-300.disabled\\:border-gray-300.transition-smooth.disabled\\:text-gray-300.bg-transparent.disabled\\:bg-transparent.text-gray-100.hover\\:text-white.underline.p-0.text-base.leading-5")).await;
    if dialog_popup.is_ok() {
        dialog_popup.unwrap().click().await?;
    }

    //this element is always found, so no need to check for err
    let text = client.find(Locator::Css(".flex.items-center.px-4")).await?.text().await?;
    if text.contains("Sign in")
    {
        log_in_artlist(client).await?;
    }

    client.goto(user_url).await?;
    tokio::time::sleep(Duration::from_secs(3)).await;

    client.find(Locator::XPath(ARTLIST_DOWNLOAD_BTN)).await?.click().await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    client.find(Locator::XPath(ARTLIST_DOWNLOAD_WAV)).await?.click().await?;

    //in the end extract user footage id for file identification on file server side
    Ok(get_footage_name(user_url))

}
