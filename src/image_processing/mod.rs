use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use crate::data;

mod s3_storage;
pub use s3_storage::S3ImageStorage;
mod sns_notifier;
pub use sns_notifier::SnsNotifier;

mod thumbnail;

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub photograph_id: uuid::Uuid,
    pub source: String,
}

pub trait Notifier : Send + Sync {
    fn notify(&self, event: Event) -> Result<(), String>;
}

pub struct ImageData {
    pub photograph_id: uuid::Uuid,
    pub image_type: data::ImageType,

    pub source: String,
    pub data: Vec<u8>,
}

pub trait ImageStorage {
    fn get(&self, key: String) -> Result<Vec<u8>, String>;
    fn set(&self, key: String, content_type: String, data: Vec<u8>) -> Result<(), String>;
}

pub trait ImageProcessor {
    fn process(&self, storage: Arc<dyn ImageStorage>, source: &ImageData) -> Result<Vec<ImageData>, String>;
}

pub struct Processor {
    storage: Arc<dyn ImageStorage>,
    data: Arc<data::Client>,
    processors: Vec<Box<dyn ImageProcessor>>,
}

impl Processor {
    pub fn new(storage: Arc<dyn ImageStorage>, data: Arc<data::Client>) -> Processor {
        let default_processors: Vec<Box<dyn ImageProcessor>> = vec!(
            Box::new(thumbnail::Thumbnail::new(None, Some(250))),
        );

        Self::new_with_processors(storage, data, default_processors)
    }

    pub fn new_with_processors(storage: Arc<dyn ImageStorage>, data: Arc<data::Client>, processors: Vec<Box<dyn ImageProcessor>>) -> Processor {
        Processor {
            storage,
            data,
            processors,
        }
    }

    pub fn process(&self, event: Event) -> Result<(), String> {
        let source = ImageData {
            photograph_id: event.photograph_id,
            image_type: data::ImageType::Full, // ?

            source: event.source.clone(),
            data: self.storage.get(event.source)?,
        };

        let mut new_images = Vec::new();

        for processor in &self.processors {
            let mut processed_images = processor.process(self.storage.clone(), &source)?;
            new_images.append(&mut processed_images);
        }

        if !new_images.is_empty() {
            self.add_new_images(event.photograph_id, new_images)?;
        }

        Ok(())
    }

    fn add_new_images(&self, id: uuid::Uuid, new_images: Vec<ImageData>) -> Result<(), String> {
        let new_images: Vec<rusoto_dynamodb::AttributeValue> = new_images.iter().map(|i|
            rusoto_dynamodb::AttributeValue {
                m: Some(serde_dynamodb::to_hashmap(&data::Image::new(i.source.clone(), i.image_type)).unwrap()),
                ..rusoto_dynamodb::AttributeValue::default()
            }
        ).collect();

        let mut updates = HashMap::new();
        updates.insert(String::from("images"), rusoto_dynamodb::AttributeValueUpdate {
            action: Some(String::from("ADD")),
            value: Some(rusoto_dynamodb::AttributeValue {
                l: Some(new_images),
                ..rusoto_dynamodb::AttributeValue::default()
            })
        });

        self.data.update_photograph(id, updates).map(|_| ())
    }
}