#![feature(proc_macro_hygiene, decl_macro)]

mod task;

use std::sync::Mutex;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod routes;
use routes::stage;

use task::{Task, NewTask};

fn rocket() -> rocket::Rocket {
    let tasks: Mutex<Vec<Task>> = Mutex::new(Vec::new());

    rocket::build()
        .manage(tasks)
        .attach(rocket_dyn_templates::Template::fairing())
        .attach(stage())
}

use rocket::fairing::AdHoc;
use rocket_contrib::templates::Template;

fn main() {
    rocket().launch();
}
