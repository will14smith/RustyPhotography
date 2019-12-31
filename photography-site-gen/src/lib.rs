use std::sync::Arc;

mod files;
mod handlebars_file;
mod model;

pub mod s3_image_provider;
pub mod s3_site_storer;

pub use files::create_files;
pub use model::*;

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