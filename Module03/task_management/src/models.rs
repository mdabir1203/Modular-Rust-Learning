use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
}