use std::fs::File;
use std::io::BufReader;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use thirtyfour::WebDriver;
use thirtyfour::Capabilities;
use eyre::Result;
use serde::Deserialize;
use thirtyfour::error::WebDriverError;

#[derive(Clone)]
pub struct Selenium {
    pub current_driver: WebDriver,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigFile {
    pub geckodriver_path:String,
    pub address: String,
    pub port: u16,
}

impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            geckodriver_path: "./src/geckodriver_mac".to_owned(),
            address: "http://localhost".to_owned(),
            port: 4444
        }
    }
}
impl ConfigFile{
    fn get_full_address(&self) -> String {
        format!("{}:{}",self.address, self.port.to_string())
    }
}

impl Selenium {
    pub async fn init_selenium_driver(config_path: &str) -> Result<Selenium, WebDriverError> {
        let config =  Self::load_webriver_config_file(config_path).unwrap_or(ConfigFile::default());
        if let Err(err) = Self::start_selenium_server(config.geckodriver_path.as_str()){
            return Err(WebDriverError::FatalError(err.to_string()))
        }

        let cap = Capabilities::new();
        match WebDriver::new(config.get_full_address(), cap).await {
            Ok(driver) => Ok(Selenium {
                // TODO web address load from config file.
                current_driver: driver,
            }),
            Err(e) => Err(WebDriverError::FatalError(e.to_string()))
        }
    }

    fn is_alive() -> bool {
        //ping selenium server
        true
    }

    pub fn load_webriver_config_file(path: &str) -> Result<ConfigFile, WebDriverError>{
        let f = File::open(path).expect("Incorrect file path");
        let reader = BufReader::new(f);
        match serde_json::from_reader(reader) {
            Ok(json) => Ok(json),
            Err(e) => Err(WebDriverError::ParseError(e.to_string()))
        }
    }

    fn start_selenium_server(geckodriver_path: &str) -> Result<()> {
        Command::new("pkill")
            .arg("geckodriver")
            .spawn()
            .expect("failed")
            .wait()
            .expect("Failed to wait"); //adjusted to wait for process to exit

        sleep(Duration::from_secs(1));

        Command::new(geckodriver_path)
            .spawn()
            .expect("gecko server (driver) process should be running");

        Ok(())
    }
}