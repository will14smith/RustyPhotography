use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Layout {
    order: i32,

    width: Option<u32>,
    height: Option<u32>,
}