mod image;
mod layout;
mod photograph;

pub use image::{Image, ImageType};
pub use layout::Layout;
pub use photograph::Photograph;

use rusoto_core::RusotoError;
use rusoto_dynamodb::DynamoDb;
use std::collections::HashMap;

pub struct Client {
    dynamo: rusoto_dynamodb::DynamoDbClient,
}

impl Client {
    pub fn new(dynamo: rusoto_dynamodb::DynamoDbClient) -> Client {
        Client {
            dynamo,
        }
    }

    pub fn list_photographs(&self) -> Result<Vec<Photograph>, RusotoError<rusoto_dynamodb::ScanError>> {
        let mut input = rusoto_dynamodb::ScanInput::default();
        input.table_name = String::from("photography-prod-photograph");

        let mut result = Vec::new();

        loop {
            let output = self.dynamo.scan(input.clone()).sync()?;
            result.extend(output.items.unwrap().into_iter().map(Photograph::from_document));

            if output.last_evaluated_key.is_none() {
                break
            }

            input.exclusive_start_key = output.last_evaluated_key;
        }

        Ok(result)
    }

    pub fn get_photograph(&self, id: uuid::Uuid) -> Result<Option<Photograph>, RusotoError<rusoto_dynamodb::GetItemError>> {
        let mut value = rusoto_dynamodb::AttributeValue::default();
        value.s = Some(format!("{}", id.to_hyphenated()));

        let mut input = rusoto_dynamodb::GetItemInput::default();
        input.table_name = String::from("photography-prod-photograph");
        input.key = HashMap::new();
        input.key.insert(String::from("id"), value);

        let output = self.dynamo.get_item(input).sync()?;

        Ok(output.item.map(Photograph::from_document))
    }
}