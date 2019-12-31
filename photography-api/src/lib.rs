#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::sync::Arc;

mod models;

mod create_photograph;
mod edit_layout;
mod edit_photograph;
mod get_photograph;
mod list_photographs;

#[get("/")]
fn index() -> &'static str {
    "This is an API! you probably don't want this page..."
}

pub fn create_api(client: Arc<photography_data::Client>, notifier: Arc<dyn photography_processing::Notifier>) -> rocket::Rocket {
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