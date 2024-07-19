use std::fs::File;
use std::io::BufReader;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use fantoccini::{Client, ClientBuilder};
use eyre::Result;
use fantoccini::error::ErrorStatus;
use fantoccini::wd::Capabilities;
use serde::Deserialize;

#[derive(Clone)]
pub struct Selenium {
    pub current_driver: Client,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigFile {
    pub geckodriver_path:String,
    pub driver_address: String,
    pub port: u16,
}

impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            geckodriver_path: "./src/geckodriver_mac".to_owned(),
            driver_address: "http://localhost".to_owned(),
            port: 4444
        }
    }
}
impl ConfigFile{
    fn get_full_address(&self) -> String {
        format!("http://{}:{}",self.driver_address, self.port.to_string())
    }
}

impl Selenium {
    pub async fn init_selenium_driver(config_path: &str) -> Result<Selenium, fantoccini::error::WebDriver> {
        let config =  Self::load_webriver_config_file(config_path).unwrap_or(ConfigFile::default());
        if let Err(err) = Self::start_selenium_server(config.geckodriver_path.as_str()){
            return Err(fantoccini::error::WebDriver::new(ErrorStatus::UnknownError ,err.to_string()))
        }

        let cap: Capabilities = serde_json::from_str(
            r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--incognito"]}}"#,
        ).unwrap();

        let mut client = ClientBuilder::native();
        match client.capabilities(cap).connect(config.get_full_address().as_str()).await {
            Ok(driver) =>  Ok(Selenium {
                    current_driver: driver
                }),
            // in case of outdated version of chrome https://googlechromelabs.github.io/chrome-for-testing/#stable
            Err(e) => Err(fantoccini::error::WebDriver::new(ErrorStatus::UnknownError ,e.to_string()))
        }
    }

    pub fn load_webriver_config_file(path: &str) -> Result<ConfigFile, fantoccini::error::WebDriver>{
        let f = File::open(path).expect("Incorrect file path");
        let reader = BufReader::new(f);
        match serde_json::from_reader(reader) {
            Ok(json) => Ok(json),
            Err(e) => Err(fantoccini::error::WebDriver::new(ErrorStatus::UnknownError ,e.to_string()))
        }
    }

    fn start_selenium_server(geckodriver_path: &str) -> Result<()> {
        // Chrome driver ends with application together, but if there is some handler err driver keeps running...
        Command::new("pkill")
            .arg("chromedri")
            .spawn()
            .expect("failed");


        let res = Command::new(geckodriver_path)
            .arg("--port=4444")
            .spawn();

        // need to give time for driver to start...
        sleep(Duration::from_secs(5));

        println!("Result is {:?}", res.err());
            // .spawn()
            // .expect("gecko server (driver) process should be running")
            // .wait()
            // .expect("Failed to wait");

        Ok(())
    }
}