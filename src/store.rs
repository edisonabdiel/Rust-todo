use std::sync::{Arc, Mutex};
use crate::task::Task;

pub struct TaskStore {
    tasks: Arc<Mutex<Vec<Task>>>,
}

impl TaskStore {
    pub fn new() -> TaskStore {
        TaskStore {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_task(&self, task: Task) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(task);
    }

    pub fn get_tasks(&self) -> Vec<Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks.clone()
    }
}
