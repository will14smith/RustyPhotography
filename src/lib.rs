#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::Serialize;
use rocket::response::status;
use rocket_contrib::json::Json;

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

pub fn create_rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, echo])
}