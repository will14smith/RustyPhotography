use serde::Serialize;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket::State;
use crate::data::Client;
use crate::models::PhotographDto;

#[derive(Serialize)]
pub struct EchoResponse {
    text: String,
}

#[get("/photograph")]
pub fn list_photographs(client: State<Client>) -> Result<Json<Vec<PhotographDto>>, status::BadRequest<String>> {
    let mut photos = client.list_photographs().map_err(|e|  status::BadRequest(Some(e.to_string())))?;
    photos.sort_by_key(|x| x.upload_time());

    let photo_dtos = photos.iter().map(|x| x.into()).collect();

    Ok(Json(photo_dtos))
}
