use std::sync::Arc;
use rocket::{ State, http::Status };
use rocket_contrib::json::Json;
use crate::models::PhotographDto;

#[get("/photograph/<id>")]
pub fn get_photograph(client: State<Arc<photography_data::Client>>, id: String) -> Result<Json<PhotographDto>, Status> {
    let id = uuid::Uuid::parse_str(&id).map_err(|_| Status::BadRequest)?;

    let photo = client.get_photograph(id).map_err(|e| {
        eprintln!("Error getting photograph: {:?}", e);

        Status::BadRequest
    })?;

    match photo {
        Some(photo) => {
            Ok(Json((&photo).into()))
        },
        None => Err(rocket::http::Status::NotFound),
    }
}
