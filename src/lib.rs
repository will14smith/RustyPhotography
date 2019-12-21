#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod config;
pub mod data;

mod models;

mod create_photograph;
mod get_photograph;
mod list_photographs;

use serde::Serialize;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::data::Client;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize)]
struct EchoResponse {
    text: String,
}

#[get("/echo/<text>")]
fn echo(text: &rocket::http::RawStr) -> Result<Json<EchoResponse>, status::BadRequest<String>> {
    let decoded_text = text.url_decode().map_err(|e| status::BadRequest(Some(e.to_string())))?;

    Ok(Json(EchoResponse{ text: decoded_text }))
}

pub fn create_rocket(client: Client) -> rocket::Rocket {
    rocket::ignite()
        .manage(client)
        .mount("/", routes![
            index, echo,
            create_photograph::create_photograph,
            get_photograph::get_photograph,
            list_photographs::list_photographs,
        ])
}