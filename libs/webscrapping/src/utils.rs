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

pub fn find_url_name(user_url: &str, offset: usize) -> Option<String> {
    let href = format!("{}", &user_url[offset..]);
    if let Some(posfix) = href.find("/")
    {
        let mut end = format!("{}", &href[..posfix]);
        if end.to_lowercase().contains("-")
        {
            //everything is converted to whitespace
            end = end.replace("-", " ").replace("--", "  ").replace("   ", "   ");
            if end.to_lowercase().contains("   ")
            {
                // 3 place whitespace is converted into " - "
                end = end.replace("   ", " - ");
                if end.to_lowercase().contains("  ")
                {
                    end = end.replace("  ", " ");
                    Some(end)
                }
                else
                {
                    Some(end)
                }
            }
            else if end.to_lowercase().contains("  ")
            {
                Some(end.replace("  ", " "))
            }
            else
            {
                Some(end)
            }
        }
        else
        {
            Some(end)
        }
        //file_helper crate that take care of upper case characters
    }
    else
    {
        None
    }
}

/// Artgrid download file with first Uppercase letter and instead of - using whitespace so in oder
/// to find download file in directory we have to change name obtained from user URL
pub fn get_footage_name(user_url: &str) -> String {
    if let Some(index) = user_url.find("song/")
    {
        // song/ == 5 offset
        find_url_name(user_url, index + 5).unwrap_or_else(|| "Invalid URL: No slash after 'clip/'".to_string())
    }
    else if let Some(index) = user_url.find("track/")
    {
        // song/ == 6 offset
        find_url_name(user_url, index + 6).unwrap_or_else(|| "Invalid URL: No slash after 'track/'".to_string())
    }
    else
    {
        "URL doesn't contain valid URL".to_string()
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

