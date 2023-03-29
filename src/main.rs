#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_dyn_templates;
extern crate tera;

use rocket::fairing::AdHoc;
use rocket_dyn_templates::Template;

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(AdHoc::on_attach("Tera Config", |rocket| {
            let tera = rocket.state::<Template>().unwrap().tera.clone();
            let tera = tera.lock().expect("tera instance is poisoned");

            Ok(rocket)
        }))
        .launch();
}
