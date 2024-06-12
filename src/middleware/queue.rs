

// saving task into queue
use crate::middleware::job::Task;
use core::ops::Deref;
use core::ops::DerefMut;
pub struct Queue {
    inner : Vec<Task>,
    //capacity: u8,
}

// we need FIFI
impl Queue {
    // we need 2 queue. One for bg task and second one for incoming user req
    pub fn new() -> Self {
        Queue {
            inner: Vec::new()
        }
    }

    pub fn enqueue(&mut self, task: Task) {
        self.inner.push(task)
    }

    pub fn dequeue(&mut self) -> Option<Task> {
        self.inner.pop()
    }
}

impl Deref for Queue {
    type Target = Task;
    fn deref(&self) -> &Task {
        if self.inner.is_empty() {
            // TODO error handling
            panic!("Queue is empty")
        }
        &self.inner[0]
    }
}

// in case we would like to modify queue tasks
impl DerefMut for Queue {
    fn deref_mut(&mut self) -> &mut Task {
         &mut self.inner[0]
    }
}