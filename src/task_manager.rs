
pub struct TaskManager {
    task: Task
}

pub struct Task {
    pub name: String,
    pub description: String
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager { task: Task{name: String::from(""), description: String::from("")} }
    }

    pub fn set_current_task(mut self, task: Task) {
        self.task = task;
    }
}