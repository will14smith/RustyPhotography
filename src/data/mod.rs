mod image;
mod layout;
mod photograph;

pub use image::{Image, ImageType};
pub use layout::Layout;
pub use photograph::Photograph;

use rusoto_dynamodb::{DynamoDb, AttributeValueUpdate};
use std::collections::HashMap;

pub struct Config {
    pub photograph_table: String,
}

pub struct Client {
    dynamo: rusoto_dynamodb::DynamoDbClient,
    config: Config,
}

fn get_photograph_key(id: uuid::Uuid) -> HashMap<String, rusoto_dynamodb::AttributeValue> {
    let mut key = HashMap::new();

    key.insert(String::from("id"), rusoto_dynamodb::AttributeValue {
        s: Some(format!("{}", id.to_hyphenated())),
        ..rusoto_dynamodb::AttributeValue::default()
    });

    key
}

impl Client {
    pub fn new(dynamo: rusoto_dynamodb::DynamoDbClient, config: Config) -> Client {
        Client {
            dynamo,
            config,
        }
    }

    pub fn list_photographs(&self) -> Result<Vec<Photograph>, String> {
        let mut input = rusoto_dynamodb::ScanInput {
            table_name: self.config.photograph_table.clone(),
            ..rusoto_dynamodb::ScanInput::default()
        };

        let mut result = Vec::new();

        loop {
            let output = self.dynamo.scan(input.clone()).sync().map_err(|e| e.to_string())?;
            result.extend(output.items.unwrap().into_iter().map(Photograph::from_document));

            if output.last_evaluated_key.is_none() {
                break
            }

            input.exclusive_start_key = output.last_evaluated_key;
        }

        Ok(result)
    }

    pub fn get_photograph(&self, id: uuid::Uuid) -> Result<Option<Photograph>, String> {
        let input = rusoto_dynamodb::GetItemInput {
            table_name: self.config.photograph_table.clone(),
            key: get_photograph_key(id),
            ..rusoto_dynamodb::GetItemInput::default()
        };

        let output = self.dynamo.get_item(input).sync().map_err(|e| e.to_string())?;

        Ok(output.item.map(Photograph::from_document))
    }

    pub fn add_photograph(&self, photograph: Photograph) -> Result<Photograph, String> {
        let input = rusoto_dynamodb::PutItemInput {
            table_name: self.config.photograph_table.clone(),
            item: photograph.to_document(),
            ..rusoto_dynamodb::PutItemInput::default()
        };

        self.dynamo.put_item(input).sync().map_err(|e| e.to_string())?;

        Ok(photograph)
    }

    pub fn update_photograph(&self, id: uuid::Uuid, updates: HashMap<String, AttributeValueUpdate>) -> Result<Photograph, String> {
        let input = rusoto_dynamodb::UpdateItemInput {
            table_name: self.config.photograph_table.clone(),
            key: get_photograph_key(id),
            attribute_updates: Some(updates),
            return_values: Some(String::from("ALL_NEW")),
            ..rusoto_dynamodb::UpdateItemInput::default()
        };

        let output = self.dynamo.update_item(input).sync().map_err(|e| e.to_string())?;

        Ok(Photograph::from_document(output.attributes.unwrap()))
    }

    pub fn update_layouts(&self, layout: HashMap<uuid::Uuid, Layout>) -> Result<(), String> {
        // TODO don't need to get all the fields
        let photographs = self.list_photographs()?;
        let ids = photographs.iter().map(|x| x.id());

        for id in ids {
            self.update_layout(id, layout.get(&id))?;
        }

        Ok(())
    }

    pub fn update_layout(&self, id: uuid::Uuid, layout: Option<&Layout>) -> Result<Photograph, String> {
        let mut updates = HashMap::new();

        let (map_val, null_val) = match layout {
            Some(l) => (Some(serde_dynamodb::to_hashmap(&l).map_err(|e| e.to_string())?), None),
            None => (None, Some(true)),
        };

        // TODO remove hard coding?
        updates.insert(String::from("layout"), rusoto_dynamodb::AttributeValueUpdate {
            action: None,
            value: Some(rusoto_dynamodb::AttributeValue {
                null: null_val,
                m: map_val,
                ..rusoto_dynamodb::AttributeValue::default()
            })
        });

        self.update_photograph(id, updates)
    }
}