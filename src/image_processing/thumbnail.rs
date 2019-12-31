use crate::image_processing::{ImageProcessor, ImageStorage, ImageData};
use std::sync::Arc;
use image::GenericImageView;

pub struct Thumbnail {
    width: Option<u32>,
    height: Option<u32>,
}

impl Thumbnail {
    pub fn new(width: Option<u32>, height: Option<u32>) -> Thumbnail {
        Thumbnail {
            width,
            height,
        }
    }

    fn calculate_dimensions(&self, original_width: u32, original_height: u32) -> (u32, u32) {
        match (self.width, self.height) {
            (Some(width), Some(height)) => { (width, height) }, // TODO check ratio and crop?
            (Some(width), None) => {
                let ratio = width as f64 / original_width as f64;
                (width, (original_height as f64 * ratio) as u32)
            },
            (None, Some(height)) => {
                let ratio = height as f64 / original_height as f64;
                ((original_width as f64 * ratio) as u32, height)
            },
            (None, None) => panic!("Width and/or Height must be set"),
        }
    }
}

impl ImageProcessor for Thumbnail {
    fn process(&self, storage: Arc<dyn ImageStorage>, source: &ImageData) -> Result<Vec<ImageData>, String> {
        let img = image::load_from_memory(&source.data).map_err(|e| e.to_string())?;
        let (width, height) = self.calculate_dimensions(img.width(), img.height());
        let resized = img.resize_exact(width, height, image::CatmullRom);

        let mut w = Vec::new();
        resized.write_to(&mut w, image::JPEG).map_err(|e| e.to_string())?;

        let key = format!("thumbnail/{}", uuid::Uuid::new_v4().to_simple());
        storage.set(key.clone(), String::from("image/jpeg"), w)?;

        Ok(vec!(ImageData {
            photograph_id: source.photograph_id,
            image_type: crate::data::ImageType::Thumbnail,

            source: key,
            data: Vec::default(),
        }))
    }
}