use crate::middleware::job::Task;
use tokio::sync::mpsc;
use tokio::task;
// bg thread controller
use lazy_static::lazy_static;
use std::ops::DerefMut;
use std::sync::Mutex;

// singletone instance, which we use accross program.
// during runtime it fill be filled with current task, that needs to be proceeds.
lazy_static! {
    static ref BG_CONTROLLER: Mutex<Option<BgController>> = Mutex::new(None);
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


    pub fn has_no_job(self) -> bool {
        if self.curr_job.is_some() {
            return true;
        }
        return false;
    }
}
// handle runnig of artgrid script and parsing url
pub fn parse_url(url: String) {
    // TODO match url

    //
}
