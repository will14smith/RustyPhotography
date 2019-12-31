use std::{ io::prelude::*, sync::Arc };
use crate::ImageStorage;

pub struct S3ImageStorage {
    bucket_name: String,
    s3: Arc<dyn rusoto_s3::S3>,
}

impl S3ImageStorage {
    pub fn new(s3: Arc<dyn rusoto_s3::S3>, bucket_name: String) -> S3ImageStorage {
        S3ImageStorage {
            s3,
            bucket_name,
        }
    }
}

impl ImageStorage for S3ImageStorage {
    fn get(&self, key: String) -> Result<Vec<u8>, String> {
        let req = rusoto_s3::GetObjectRequest {
            bucket: self.bucket_name.clone(),
            key,

            ..rusoto_s3::GetObjectRequest::default()
        };

        let res = self.s3.get_object(req).sync().map_err(|e| e.to_string())?;
        let str = res.body.ok_or("Failed to get object body")?;

        let mut result = Vec::new();
        str.into_blocking_read().read_to_end(&mut result).map_err(|e| e.to_string())?;

        Ok(result)
    }

    fn set(&self, key: String, content_type: String, data: Vec<u8>) -> Result<(), String> {
        let req = rusoto_s3::PutObjectRequest{
            bucket: self.bucket_name.clone(),
            key,
            content_type: Some(content_type),

            body: Some(data.into()),

            ..rusoto_s3::PutObjectRequest::default()
        };

        self.s3.put_object(req).sync().map_err(|e| e.to_string())?;

        Ok(())
    }
}