use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    object_key: String,

    #[serde(rename = "type")]
    image_type: ImageType,
}

impl Image {
    pub fn object_key(&self) -> &String { &self.object_key }
    pub fn image_type(&self) -> ImageType { self.image_type }
}

#[derive(Copy, Clone, Debug)]
pub enum ImageType {
    Full,
    Thumbnail,
}

impl Serialize for ImageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(match *self {
            ImageType::Full => "Full",
            ImageType::Thumbnail => "Thumbnail",
        })
    }
}

impl<'de> Deserialize<'de> for ImageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Full" => Ok(ImageType::Full),
            "Thumbnail" => Ok(ImageType::Thumbnail),
            // TODO make this return an error instead
            _ => panic!("Invalid enum value: {}", s),
        }
    }
}