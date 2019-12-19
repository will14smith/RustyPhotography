mod image;
mod layout;
mod photograph;

pub use photograph::Photograph;

use rusoto_core::RusotoError;
use rusoto_dynamodb::{DynamoDb, ScanInput, ScanError};

pub struct Client<'a> {
    dynamo: &'a dyn DynamoDb,
}

impl<'a> Client<'a> {
    pub fn new(dynamo: &dyn DynamoDb) -> Client {
        Client {
            dynamo
        }
    }

    pub fn list_photographs(&self) -> Result<Vec<Photograph>, RusotoError<ScanError>> {
        let mut input = ScanInput::default();
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
}