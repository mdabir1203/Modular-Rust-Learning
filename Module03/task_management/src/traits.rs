// src/traits.rs

use crate::models::{Task, TaskStatus};

pub trait TaskManager {
    fn add_task(&mut self, task: Task);
    fn update_task_status(&mut self, task_id: u32, status: TaskStatus);
    fn list_tasks(&self) -> Vec<&Task>;
}
