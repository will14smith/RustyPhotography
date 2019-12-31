use std::sync::Arc;
use rocket::{ State, http::Status };
use rocket_contrib::json::Json;
use serde::Deserialize;
use crate::models::PhotographDto;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreatePhotographDto {
    title: String,

    image_key: String,

    capture_time: chrono::DateTime<chrono::Utc>,
}

impl Into<photography_data::Photograph> for CreatePhotographDto {
    fn into(self) -> photography_data::Photograph {
        use photography_data::{Photograph, Image, ImageType};

        let mut photograph = Photograph::new();

        photograph
            .set_title(self.title.clone())
            .add_image(Image::new(self.image_key.clone(), ImageType::Full))
            .set_capture_time(self.capture_time);

        photograph
    }
}

#[post("/photograph", format = "json", data = "<input>")]
pub fn create_photograph(client: State<Arc<photography_data::Client>>, notifier: State<Arc<dyn photography_processing::Notifier>>, input: Json<CreatePhotographDto>) -> Result<Json<PhotographDto>, Status> {
    let photograph = input.into_inner().into();

    let photograph = client.add_photograph(photograph).map_err(|e| {
        eprintln!("Failed to add photograph: {:?}", e);

        Status::InternalServerError
    })?;

    notifier.notify(photography_processing::Event {
        photograph_id: photograph.id(),
        source: photograph.images().get(0).unwrap().object_key().clone(),
    }).map_err(|e| {
        eprintln!("Failed to send processing notification: {:?}", e);

        Status::InternalServerError
    })?;

    Ok(Json((&photograph).into()))
}