use std::collections::HashMap;
use serde::Deserialize;
use rocket::{ State, http::Status };
use rocket_contrib::json::Json;
use crate::data::{Client, Layout};
use std::sync::Arc;

#[derive(Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EditLayoutsDto(HashMap<uuid::Uuid, EditLayoutDto>);

#[derive(Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EditLayoutDto {
    order: i32,

    width: Option<u32>,
    height: Option<u32>,
}

impl Into<Layout> for &EditLayoutDto {
    fn into(self) -> Layout {
        Layout::new(self.order, self.width, self.height)
    }
}

#[put("/layout", format = "json", data = "<input>")]
pub fn edit_layout(client: State<Arc<Client>>, input: Json<EditLayoutsDto>) -> Result<Status, Status> {
    let model = input.into_inner().0.iter()
        .map(|(&k, v)| (k, v.into()))
        .collect();

    client.update_layouts(model).map_err(|e| {
        eprintln!("Error saving layout: {:?}", e);

        Status::BadRequest
    })?;

    Ok(Status::Accepted)
}