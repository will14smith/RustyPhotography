use std::sync::Arc;
use photography_data::Photograph;
use serde::{ Serialize, Serializer, ser::SerializeMap };

pub trait SiteFile {
    fn name(&self) -> &String;
    fn content_type(&self) -> &String;

    fn generate(&self) -> String;
}

pub struct Site {
    pub files: Vec<Box<dyn SiteFile>>,
}

pub struct PhotographModel {
    photograph: Arc<Photograph>,
    thumbnail_url: String,
}

impl PhotographModel {
    pub fn new(photograph: Arc<Photograph>, thumbnail_url: String) -> PhotographModel {
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
