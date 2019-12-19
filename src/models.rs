use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::data::{Photograph, ImageType, Image, Layout};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PhotographDto {
    id: Uuid,

    title: String,

    layout: Option<LayoutDto>,

    images: Vec<ImageDto>,

    capture_time: chrono::DateTime<chrono::Utc>,
    upload_time: chrono::DateTime<chrono::Utc>,
}

impl From<&Photograph> for PhotographDto {
    fn from(p: &Photograph) -> Self {
        PhotographDto {
            id: p.id(),

            title: p.title().to_owned(),

            layout: p.layout().as_ref().map(|x| x.into()),

            images: p.images().iter().map(|x| x.into()).collect(),

            capture_time: p.capture_time(),
            upload_time: p.upload_time(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ImageDto {
    object_key: String,

    #[serde(rename = "Type")]
    image_type: ImageType,
}

impl From<&Image> for ImageDto {
    fn from(i: &Image) -> Self {
        ImageDto {
            object_key: i.object_key().to_owned(),

            image_type: i.image_type(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LayoutDto {
    order: i32,

    width: Option<u32>,
    height: Option<u32>,
}

impl From<&Layout> for LayoutDto {
    fn from(l: &Layout) -> Self {
        LayoutDto {
            order: l.order(),

            width: l.width(),
            height: l.height(),
        }
    }
}