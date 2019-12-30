use crate::site_gen::{SiteStorer, Site, SiteFile};
use std::sync::Arc;

pub struct S3SiteStorer {
    bucket_name: String,
    s3: Arc<dyn rusoto_s3::S3>,
}

impl S3SiteStorer {
    pub fn new(bucket_name: String, s3: Arc<dyn rusoto_s3::S3>) -> S3SiteStorer {
        S3SiteStorer {
            bucket_name,
            s3,
        }
    }

    fn store_file(&self, file: &Box<dyn SiteFile>) -> Result<(), String> {
        let req = rusoto_s3::PutObjectRequest{
            bucket: self.bucket_name.clone(),
            key: file.name().clone(),
            content_type: Some(file.content_type().clone()),

            body: Some(file.generate().into_bytes().into()),

            ..rusoto_s3::PutObjectRequest::default()
        };

        self.s3.put_object(req).sync().map_err(|e| e.to_string())?;

        Ok(())
    }
}

impl SiteStorer for S3SiteStorer {
    fn store(&self, site: Site) -> Result<(), String> {
        for file in site.files() {
            self.store_file(file)?;
        }

        Ok(())
    }
}