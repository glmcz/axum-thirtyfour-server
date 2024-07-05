use crate::middleware::job::Task;
use tokio::sync::mpsc;
use eyre::{Ok, Result};
// bg thread controller
use lazy_static::lazy_static;
use std::sync::Mutex;

use super::cpu::{CPU, init_cpu_measurement};
//use crate::footager::artgrid;
use crate::Selenium;


// singletone instance, which we use accross program.
// during runtime it fill be filled with current task, that needs to be proceeds.
lazy_static! {
    static ref BG_CONTROLLER: Mutex<Option<BgController>> = Mutex::new(None);
    static ref METRICS: Mutex<CPU> = Mutex::new(init_cpu_measurement()); 
}
pub struct BgController {
    curr_job: Option<Task>, // we should have only one task at one time. We can put it into queue etc...
    tx: Option<mpsc::Sender<Task>>, // send req to main thread queue that we need another task
    rx: Option<mpsc::Receiver<Task>>, // receive main thread Task
}
// object is init if a new req come until last req, so no pernamently runnig

// putting tasks int queue and taking care of cpu usage
impl BgController {
    pub fn init() -> Self {
        BgController {
            curr_job: None,
            tx: None,
            rx: None,
        }
    }

    pub fn get_instance() -> Option<BgController> {
        let mut guard = BG_CONTROLLER.lock().unwrap();
        // check if instance has some Task
        guard.take()
    }

    // if curr_job is some add job into queue
    // in next phrase check demanding of CPU and mem
    // and according to it allow more async current jobs 
    pub fn has_no_job(&self) -> bool {
        if self.curr_job.is_some() {
            return true;
        }
        return false;
    }
    // take all jobs from footager users
    pub fn add_job(&mut self, job: Task) -> Result<(), eyre::Report> {
        // add items into the queue
        let mut guard = METRICS.lock().unwrap();
        guard.get_cpu_usage();
        guard.get_memory_usage();
        // hm currently we are proceeding only one task at time.
        if self.has_no_job() {
            // check queue and fill it 
            // or fill it with a new req directly
            self.curr_job = Some(job);
        }
        // better desing of app we need

        Ok(())
    }
}
// handle runnig of artgrid script and parsing url
pub fn parse_url(url: String) {
    // TODO match url
    log::info!("We are choosing right script for user url \n{}", url);
    
}
