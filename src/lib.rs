#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod config;
pub mod data;
pub mod site_gen;
pub mod image_processing;

mod models;

mod create_photograph;
mod edit_photograph;
mod get_photograph;
mod list_photographs;

mod edit_layout;

use std::sync::Arc;

#[get("/")]
fn index() -> &'static str {
    "This is an API! you probably don't want this page..."
}

pub fn create_rocket(client: Arc<data::Client>, notifier: Arc<dyn image_processing::Notifier>) -> rocket::Rocket {
    rocket::ignite()
        .manage(client).manage(notifier)
        .mount("/", routes![
            index,
            create_photograph::create_photograph,
            edit_photograph::edit_photograph,
            get_photograph::get_photograph,
            list_photographs::list_photographs,

            edit_layout::edit_layout,
        ])
}