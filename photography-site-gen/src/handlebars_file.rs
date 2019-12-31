use std::rc::Rc;
use chrono::Datelike;
use handlebars::{Handlebars, to_json};
use serde_json::value::{Map, Value as Json};
use crate::SiteFile;

pub struct HandlebarsFile {
    handlebars: Rc<Handlebars>,

    name: String,
    content_type: String,
    data: Map<String, Json>,
}

impl HandlebarsFile {
    pub fn new(handlebars: Rc<Handlebars>, name: &str, content_type: &str, data: Map<String, Json>) -> HandlebarsFile {
        let mut data = data;
        data.insert("year".into(), to_json(chrono::Utc::today().year()));

        HandlebarsFile {
            handlebars,

            name: String::from(name),
            content_type: String::from(content_type),
            data,
        }
    }
}

impl SiteFile for HandlebarsFile {
    fn name(&self) -> &String {
        &self.name
    }

    fn content_type(&self) -> &String {
        &self.content_type
    }

    fn generate(&self) -> String {
        self.handlebars.render(&self.name, &self.data).unwrap()
    }
}