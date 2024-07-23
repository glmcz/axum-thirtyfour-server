use std::path::Path;
use std::process::Command;
use fantoccini::ClientBuilder;
use fantoccini::wd::Capabilities;

pub fn get_href_value(user_url: &str) -> String {
    if let Some(index) = user_url.find("/") {
        let href = format!(
            "{}{}{}",
            String::from("//a[@href='"),
            &user_url[index + 12..],
            "']"
        );
        href
    } else {
        "String is not a valid URL".to_string()
    }
    // Error: NoSuchElement("XPath(//a[@href='https://artgrid.io/clip/302105/boat-river-buildings-clouds'])")
}

pub fn get_footage_id(user_url: &str) -> String {
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

pub async fn init_webdriver() -> fantoccini::Client {
    Command::new("pkill")
        .arg("chromedri")
        .spawn()
        .expect("failed");

    let path = Path::new("chromedriver");
    Command::new(path)
        .arg("--port=4444")
        .spawn()
        .expect("chrome driver is running");

    let cap: Capabilities = serde_json::from_str(
        r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--incognito"]}}"#,
    ).unwrap();
    //cap.insert("", "");
    let mut client = ClientBuilder::native();

    client.capabilities(cap).connect("http://localhost:4444").await.expect("localhost init failed")
}

