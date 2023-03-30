use rocket::http::Status;
use rocket::request::Form;
use rocket_contrib::json::Json;


use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde::Serialize;
use rocket_contrib::json;

use crate::task::{Task, NewTask};


use rocket::State;
use std::sync::Mutex;

#[derive(Debug, Serialize)]
struct Context {
    title: String,
    tasks: Vec<Task>,
}

#[derive(Debug, Serialize)]
struct NewContext {
    title: String,
}

#[derive(Debug, Serialize)]
struct DeleteContext {
    title: String,
    id: u32,
}

#[get("/")]
pub fn index(tasks: &State<Mutex<Vec<Task>>>) -> Template {
    let tasks = tasks.lock().unwrap();
    let context = json!({ "tasks": &*tasks });
    Template::render("index", context)
}

#[post("/", data = "<new_task>")]
fn add_task(new_task: Form<NewTask>, tasks: &State<Mutex<Vec<Task>>>) {
    let mut tasks = tasks.lock().unwrap();
    tasks.push(Task {
        id: tasks.len() + 1,
        description: new_task.into_inner().description,
        completed: false,
    });
}

#[put("/<id>")]
fn complete_task(id: u32, tasks: &State<Mutex<Vec<Task>>>) {
    let mut tasks = tasks.lock().unwrap();
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.completed = true;
    }
}

#[post("/create", format = "application/json", data = "<todo_form>")]
pub fn create(todo_form: Json<TodoForm>, tasks: &State<Mutex<Vec<Task>>>) -> Redirect {
    let task = Task::new(todo_form.into_inner().description);
    tasks.lock().unwrap().push(task);
    Redirect::to("/")
}

#[delete("/<id>")]
fn delete_task(id: u32, tasks: &State<Mutex<Vec<Task>>>) {
    let mut tasks = tasks.lock().unwrap();
    tasks.retain(|task| task.id != id);
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Template Context", |rocket| async {
        let rocket = rocket.mount("/", rocket::routes![index, add_task, complete_task, create, delete_task]);
        Ok(rocket)
    })
}
