use std::sync::Arc;
use crate::{Notifier, Event};

pub struct SnsNotifier {
    sns: Arc<dyn rusoto_sns::Sns + Send + Sync>,
    topic_arn: String,
}

impl SnsNotifier {
    pub fn new(sns: Arc<dyn rusoto_sns::Sns + Send + Sync>, topic_arn: String) -> SnsNotifier {
        SnsNotifier {
            sns,
            topic_arn,
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