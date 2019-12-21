use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::{ State, http::Status };
use crate::data::{Client, Photograph, Image, ImageType};
use crate::models::PhotographDto;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreatePhotographDto {
    title: String,

    image_key: String,

    capture_time: chrono::DateTime<chrono::Utc>,
}

impl Into<Photograph> for CreatePhotographDto {
    fn into(self) -> Photograph {
        let mut photograph = Photograph::new();

        photograph
            .set_title(self.title.clone())
            .add_image(Image::new(self.image_key.clone(), ImageType::Full))
            .set_capture_time(self.capture_time);

        photograph
    }
}

#[post("/photograph", format = "json", data = "<input>")]
pub fn create_photograph(client: State<Client>, input: Json<CreatePhotographDto>) -> Result<Json<PhotographDto>, Status> {
    let photograph = input.into_inner().into();

    let photograph = client.add_photograph(photograph).map_err(|e| {
        eprintln!("Failed to add photograph: {:?}", e);

        Status::InternalServerError
    })?;

    Ok(Json((&photograph).into()))
}