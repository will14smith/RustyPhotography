use std::sync::Arc;
use rocket::{ response::status, State };
use rocket_contrib::json::Json;
use crate::models::PhotographDto;

#[get("/photograph")]
pub fn list_photographs(client: State<Arc<photography_data::Client>>) -> Result<Json<Vec<PhotographDto>>, status::BadRequest<String>> {
    let mut photos = client.list_photographs().map_err(|e| {
        eprintln!("Error getting photograph: {:?}", e);

        status::BadRequest(Some(e.to_string()))
    })?;
    photos.sort_by_key(|x| x.upload_time());

    let photo_dtos = photos.iter().map(|x| x.into()).collect();

    Ok(Json(photo_dtos))
}
