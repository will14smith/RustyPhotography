use std::rc::Rc;
use serde::ser::{Serialize, Serializer, SerializeMap };
use crate::data::Photograph;

mod files;
mod handlebars_file;
pub mod image_provider;
pub mod site_storer;

pub use files::create_files;
use std::sync::Arc;

pub struct Site {
    files: Vec<Box<dyn SiteFile>>,
}

impl Site {
    pub fn files(&self) -> &Vec<Box<dyn SiteFile>> { &self.files }
}

pub struct PhotographModel {
    photograph: Rc<Photograph>,
    thumbnail_url: String,
}

impl PhotographModel {
    pub fn new(photograph: Rc<Photograph>, thumbnail_url: String) -> PhotographModel {
        PhotographModel{
            photograph,
            thumbnail_url,
        }
    }

    pub fn column_width(&self) -> u32 {
        self.photograph.layout().as_ref()
            .and_then(|l| l.width())
            .unwrap_or(1) * 4
    }
}

impl Serialize for PhotographModel {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer
    {
        let mut map = serializer.serialize_map(Some(3))?;

        map.serialize_entry("photograph", self.photograph.as_ref())?;
        map.serialize_entry("thumbnail_url", &self.thumbnail_url)?;
        map.serialize_entry("column_width", &self.column_width())?;

        map.end()
    }
}

pub trait SiteFile {
    fn name(&self) -> &String;
    fn content_type(&self) -> &String;

    fn generate(&self) -> String;
}

pub trait ImageProvider {
    fn get_photos(&self) -> Result<Vec<PhotographModel>, String>;
}

pub trait SiteStorer {
    fn store(&self, site: Site) -> Result<(), String>;
}

pub fn generate(image_provider: Arc<dyn ImageProvider>, site_storer: Arc<dyn SiteStorer>, template_path: &str) -> Result<(), String> {
    let images = image_provider.get_photos()?;
    let files = files::create_files(images, template_path);

    let site = Site { files };
    site_storer.store(site)
}