use lambda_runtime::{
    Context,
    error::HandlerError,
    lambda,
};
use serde::{ Deserialize, Serialize };

fn main() {
    lambda!(handler);
}

#[derive(Deserialize)]
struct Event {}
#[derive(Serialize)]
struct Output {}

fn handler(_: Event, _: Context) -> Result<Output, HandlerError> {
    let config = photography_lambda::Config::from_env().expect("failed to get config");

    let image_provider = photography_lambda::get_image_provider(&config);
    let site_storer = photography_lambda::get_site_storer(&config);

    photography_site_gen::generate(image_provider, site_storer, "/opt").map_err(|e| {
        HandlerError::from(e.as_str())
    })?;

    Ok(Output {})
}