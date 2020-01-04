use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    pub photograph_id: uuid::Uuid,
    pub image: EventImage,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventImage {
    pub object_key: String,

    #[serde(rename = "type")]
    pub image_type: photography_data::ImageType,
}

pub struct ImageData {
    pub photograph_id: uuid::Uuid,
    pub image_type: photography_data::ImageType,

    pub source: String,
    pub data: Vec<u8>,
}
