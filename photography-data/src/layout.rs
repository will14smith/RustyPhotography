use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Layout {
    order: i32,

    width: Option<u32>,
    height: Option<u32>,
}

impl Layout {
    pub fn new(order: i32, width: Option<u32>, height: Option<u32>) -> Layout {
        Layout {
            order,
            width,
            height,
        }
    }

    pub fn order(&self) -> i32 { self.order }
    pub fn width(&self) -> Option<u32> { self.width }
    pub fn height(&self) -> Option<u32> { self.height }
}
