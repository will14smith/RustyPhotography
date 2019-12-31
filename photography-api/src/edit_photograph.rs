use std::{ collections::HashMap, sync::Arc };
use rocket_contrib::json::Json;
use rocket::{ State, http::Status };
use serde::Deserialize;
use crate::models::PhotographDto;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EditPhotographDto {
    title: String,

    capture_time: chrono::DateTime<chrono::Utc>,
}

impl EditPhotographDto {
    fn get_update_values(self) -> HashMap<String, rusoto_dynamodb::AttributeValueUpdate> {
        let mut photograph = photography_data::Photograph::new();
        photograph
            .set_title(self.title)
            .set_capture_time(self.capture_time);

        let attr_values = serde_dynamodb::to_hashmap(&photograph).unwrap();

        // TODO this is "hard-coded" to the field names
        let mut updates = HashMap::new();
        updates.insert(String::from("title"), rusoto_dynamodb::AttributeValueUpdate { action: None, value: attr_values.get("title").map(|x| x.clone()) });
        updates.insert(String::from("captureTime"), rusoto_dynamodb::AttributeValueUpdate { action: None, value: attr_values.get("captureTime").map(|x| x.clone()) });

        updates
    }
}

#[put("/photograph/<id>", format = "json", data = "<input>")]
pub fn edit_photograph(client: State<Arc<photography_data::Client>>, id: String, input: Json<EditPhotographDto>) -> Result<Json<PhotographDto>, Status> {
    let id = uuid::Uuid::parse_str(&id).map_err(|_| Status::BadRequest)?;

    let updates = input.into_inner().get_update_values();

    let photograph = client.update_photograph(id, updates).map_err(|e| {
        eprintln!("Failed to update photograph: {:?}", e);

        Status::InternalServerError
    })?;

    Ok(Json((&photograph).into()))
}