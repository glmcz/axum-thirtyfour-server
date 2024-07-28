use std::process::Command;
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;
use fantoccini::{Client, ClientBuilder};
use fantoccini::error::ErrorStatus;
use fantoccini::wd::Capabilities;
use serde::Deserialize;
pub mod artgrid;
pub mod artlist;
pub mod utils;
pub mod domain;



#[derive(Clone)]
pub struct Selenium {
    pub current_driver: Option<Client>,
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


pub trait SeleniumOperations: Send + Sync + 'static{
    async fn init_connection(&mut self, config_file: ConfigFile) -> Result<(), fantoccini::error::WebDriver>;
    fn get_driver(&self) -> Option<&Client>;
}


impl SeleniumOperations for Selenium {
    async fn init_connection(&mut self, config: ConfigFile) -> Result<(), fantoccini::error::WebDriver> {
        // a new_tab doesn't work, because of Chromedriver bug in incognito mode
        // r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--incognito"]}}"#,
        let cap: Capabilities = serde_json::from_str(
            r#"{"browserName":"chrome"}"#,
        ).unwrap();

        let mut client = ClientBuilder::native();
        match client.capabilities(cap).connect(config.get_full_address().as_str()).await {
            Ok(driver) =>  Ok( self.current_driver = Some(driver)
            ),
            // in case of outdated version of chrome https://googlechromelabs.github.io/chrome-for-testing/#stable
            Err(e) => Err(fantoccini::error::WebDriver::new(ErrorStatus::UnknownError ,e.to_string()))
        }
    }
    fn get_driver(&self) -> Option<&Client> {
       self.current_driver.as_ref()
    }
}

impl Selenium {
    pub fn new() -> Self {
        Selenium{
            current_driver: None
        }
    }
    pub fn load_webriver_config_file(&self, path: &str) -> Result<ConfigFile, fantoccini::error::WebDriver>{
        let f = File::open(path).expect("Incorrect file path");
        let reader = BufReader::new(f);
        match serde_json::from_reader(reader) {
            Ok(json) => Ok(json),
            Err(e) => Err(fantoccini::error::WebDriver::new(ErrorStatus::UnknownError ,e.to_string()))
        }
    }

    pub fn start_selenium_server(geckodriver_path: &str) -> Result<(), fantoccini::error::WebDriver> {
        // Chrome driver ends with application together, but if there is some handler err driver keeps running...
        Command::new("pkill")
            .arg("chromedri")
            .spawn()
            .expect("failed");

        sleep(Duration::from_secs(1));

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

