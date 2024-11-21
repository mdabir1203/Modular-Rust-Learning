#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use crate::models::{Task, TaskStatus};
use crate::project::Project;
use crate::traits::TaskManager;
use std::sync::Mutex; // Add this import

mod models;
mod traits;
mod project;



#[post("/tasks", format = "json", data = "<task>")]
async fn add_task(task: Json<Task>, project: &State<Mutex<Project>>) {
    let mut project = project.lock().expect("Failed to lock the mutex");
    project.add_task(task.into_inner());
}

#[post("/tasks/<task_id>/status", format = "json", data = "<status>")]
async fn update_task_status(task_id: u32, status: Json<TaskStatus>, project: &State<Mutex<Project>>) {
    let mut project = project.lock().expect("Failed to lock the mutex");
    project.update_task_status(task_id, status.into_inner());
}

#[get("/tasks")]
async fn list_tasks(project: &State<Project>) -> Json<Vec<&Task>> {
    Json(project.list_tasks())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(Project::new())) // Wrap Project in Mutex and manage it
        .mount("/", routes![add_task, update_task_status, list_tasks])
}
