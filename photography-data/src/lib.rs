mod client;
mod image;
mod layout;
mod photograph;

pub use client::Client;
pub use image::{Image, ImageType};
pub use layout::Layout;
pub use photograph::Photograph;

pub struct Config {
    pub photograph_table: String,
}