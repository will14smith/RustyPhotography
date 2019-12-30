use crate::site_gen::{ImageProvider, PhotographModel};
use crate::data::{Photograph, ImageType, Client};
use std::rc::Rc;
use std::sync::Arc;
use rusoto_s3::util::PreSignedRequest;
use std::time::Duration;

pub struct S3ImageProviderConfig {
    pub bucket_name: String,
    pub region: rusoto_core::Region,
    pub credentials: rusoto_core::credential::AwsCredentials,
}

pub struct S3ImageProvider {
    client: Arc<Client>,
    config: S3ImageProviderConfig,
}

impl S3ImageProvider {
    pub fn new(client: Arc<Client>, config: S3ImageProviderConfig) -> S3ImageProvider {
        S3ImageProvider {
            client,
            config,
        }
    }

    fn get_thumbnail_url(&self, photograph: &Photograph) -> String {
        let thumbnail = photograph.images().iter()
            .filter(|p| match p.image_type() { ImageType::Thumbnail => true, _ => false })
            .nth(0);

        match thumbnail {
            Some(image) => {
                let req = rusoto_s3::GetObjectRequest {
                    bucket: self.config.bucket_name.clone(),
                    key: image.object_key().clone(),
                    ..rusoto_s3::GetObjectRequest::default()
                };

                req.get_presigned_url(&self.config.region, &self.config.credentials, &rusoto_s3::util::PreSignedRequestOption {
                    expires_in: Duration::from_secs(2 * 24 * 60 * 60),
                })
            },
            None => format!("http://via.placeholder.com/{}x233?text=Processing...", 350*photograph.layout().as_ref().unwrap().width().unwrap_or(1))
        }
    }
}

impl ImageProvider for S3ImageProvider {
    fn get_photos(&self) -> Result<Vec<PhotographModel>, String> {
        let photographs = self.client.list_photographs()?;

        let result = photographs.into_iter()
            .map(Rc::new)
            .filter(|p| p.layout().is_some())
            .map(|p| (Rc::clone(&p), self.get_thumbnail_url(&Rc::clone(&p))))
            .map(|(p, s)| PhotographModel::new(p, s))
            .collect();

        Ok(result)
    }
}