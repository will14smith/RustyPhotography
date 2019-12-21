use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use rusoto_dynamodb::AttributeValue;

use crate::data::{
    image::Image,
    layout::Layout,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Photograph {
    id: Uuid,

    title: String,

    layout: Option<Layout>,

    images: Vec<Image>,

    capture_time: chrono::DateTime<chrono::Utc>,
    upload_time: chrono::DateTime<chrono::Utc>,
}

impl Photograph {
    pub fn new() -> Photograph {
        Photograph {
            id: Uuid::new_v4(),
            title: String::new(),
            layout: None,
            images: vec![],
            capture_time: chrono::Utc::now(),
            upload_time: chrono::Utc::now(),
        }
    }

    pub fn id(&self) -> Uuid { self.id }
    pub fn title(&self) -> &String { &self.title }
    pub fn layout(&self) -> &Option<Layout> { &self.layout }
    pub fn images(&self) -> &Vec<Image> { &self.images }
    pub fn capture_time(&self) -> chrono::DateTime<chrono::Utc> { self.capture_time }
    pub fn upload_time(&self) -> chrono::DateTime<chrono::Utc> { self.upload_time }

    pub fn set_id(&mut self, id: Uuid) -> &mut Self { self.id = id; self }
    pub fn set_title(&mut self, title: String) -> &mut Self { self.title = title; self }
    pub fn set_layout(&mut self, layout: Option<Layout>) -> &mut Self { self.layout = layout; self }
    pub fn add_image(&mut self, image: Image) -> &mut Self { self.images.push(image); self }
    pub fn set_capture_time(&mut self, capture_time: chrono::DateTime<chrono::Utc>) -> &mut Self { self.capture_time = capture_time; self }
    pub fn set_upload_time(&mut self, upload_time: chrono::DateTime<chrono::Utc>) -> &mut Self { self.upload_time = upload_time; self }

    pub fn from_document(document: HashMap<String, AttributeValue>) -> Photograph {
        serde_dynamodb::from_hashmap(document).unwrap()
    }

    pub fn to_document(&self) -> HashMap<String, AttributeValue> {
        serde_dynamodb::to_hashmap(self).unwrap()
    }
}
