use std::{ collections::HashMap, sync::Arc };
use rocket::{ State, http::Status };
use rocket_contrib::json::Json;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EditLayoutsDto(HashMap<uuid::Uuid, EditLayoutDto>);

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EditLayoutDto {
    order: i32,

    width: Option<u32>,
    height: Option<u32>,
}

impl Into<photography_data::Layout> for &EditLayoutDto {
    fn into(self) -> photography_data::Layout {
        photography_data::Layout::new(self.order, self.width, self.height)
    }
}

#[put("/layout", format = "json", data = "<input>")]
pub fn edit_layout(client: State<Arc<photography_data::Client>>, input: Json<EditLayoutsDto>) -> Result<Status, Status> {
    let model = input.into_inner().0.iter()
        .map(|(&k, v)| (k, v.into()))
        .collect();

    client.update_layouts(model).map_err(|e| {
        eprintln!("Error saving layout: {:?}", e);

        Status::BadRequest
    })?;

    Ok(Status::Accepted)
}