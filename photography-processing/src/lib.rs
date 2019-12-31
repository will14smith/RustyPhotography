use std::sync::Arc;

mod model;
mod processor;
mod s3_storage;
mod sns_notifier;
mod thumbnail;

pub use model::*;
pub use processor::*;
pub use s3_storage::S3ImageStorage;
pub use sns_notifier::SnsNotifier;

pub trait Notifier : Send + Sync {
    fn notify(&self, event: Event) -> Result<(), String>;
}

pub trait ImageStorage {
    fn get(&self, key: String) -> Result<Vec<u8>, String>;
    fn set(&self, key: String, content_type: String, data: Vec<u8>) -> Result<(), String>;
}

pub trait ImageProcessor {
    fn process(&self, storage: Arc<dyn ImageStorage>, source: &ImageData) -> Result<Vec<ImageData>, String>;
}