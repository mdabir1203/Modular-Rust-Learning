// src/project.rs

use std::collections::HashMap;
use crate::models::{Task, TaskStatus};
use crate::traits::TaskManager;

pub struct Project {
    tasks: HashMap<u32, Task>,
}

impl Project {
    pub fn new() -> Self {
        Project {
            tasks: HashMap::new(),
        }
    }
}

impl TaskManager for Project {
    fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn update_task_status(&mut self, task_id: u32, status: TaskStatus) {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.status = status;
        }
    }

    fn list_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
}
