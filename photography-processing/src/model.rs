use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub photograph_id: uuid::Uuid,
    pub source: String,
}

pub struct ImageData {
    pub photograph_id: uuid::Uuid,
    pub image_type: photography_data::ImageType,

    pub source: String,
    pub data: Vec<u8>,
}
