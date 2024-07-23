


// requirements
// make task struct which save all events for current task trigger in bg_thread
// after it is finished it pass data to queue that it can take another one

pub struct Task {
    task_name: String, // get it from name of footager user
    user_url: String,
    user_job: String,   // requested downloaded video or audio 
    finished: bool,
}

impl Task {
    pub fn init() -> Self {
        Task {
            task_name: "".into(),
            user_url: "".into(),
            user_job: "".into(),
            finished: false,
        }
    }

    pub fn new(task: String, url: String, job: String, finished: bool) -> Self {
        Task {
            task_name: task,
            user_url: url,
            user_job: job,
            finished: finished,
        }
    }

    // impl set_finished
}


