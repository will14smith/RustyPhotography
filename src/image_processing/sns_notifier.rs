use std::sync::Arc;
use crate::image_processing::{Notifier, Event};

pub struct SnsNotifier {
    topic_arn: String,
    sns: Arc<dyn rusoto_sns::Sns + Send + Sync>,
}

impl SnsNotifier {
    pub fn new(topic_arn: String, sns: Arc<dyn rusoto_sns::Sns + Send + Sync>) -> SnsNotifier {
        SnsNotifier {
            topic_arn,
            sns,
        }
    }
}

impl Notifier for SnsNotifier {
    fn notify(&self, event: Event) -> Result<(), String> {
        let message = serde_json::to_string(&event).map_err(|e| e.to_string())?;

        let req = rusoto_sns::PublishInput {
            topic_arn: Some(self.topic_arn.clone()),
            message,

            ..rusoto_sns::PublishInput::default()
        };

        self.sns.publish(req).sync().map_err(|e| e.to_string())?;

        Ok(())
    }
}