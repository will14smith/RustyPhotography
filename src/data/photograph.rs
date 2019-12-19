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
    pub fn id(&self) -> Uuid { self.id }
    pub fn title(&self) -> &String { &self.title }
    pub fn layout(&self) -> &Option<Layout> { &self.layout }
    pub fn images(&self) -> &Vec<Image> { &self.images }
    pub fn capture_time(&self) -> chrono::DateTime<chrono::Utc> { self.capture_time }
    pub fn upload_time(&self) -> chrono::DateTime<chrono::Utc> { self.upload_time }

    pub fn from_document(document: HashMap<String, AttributeValue>) -> Photograph {
        serde_dynamodb::from_hashmap(document).unwrap()
    }

    pub fn to_document(&self) -> HashMap<String, AttributeValue> {
        serde_dynamodb::to_hashmap(self).unwrap()
    }
}
