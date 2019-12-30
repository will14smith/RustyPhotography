use serde_json::Map;
use crate::site_gen::{SiteFile, PhotographModel};
use crate::site_gen::handlebars_file::HandlebarsFile;
use handlebars::{to_json, Handlebars, handlebars_helper};
use std::rc::Rc;
use chrono::TimeZone;

pub fn create_files(photographs: Vec<PhotographModel>, template_path: &str) -> Vec<Box<dyn SiteFile>> {
    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".hbs", template_path).unwrap();

    handlebars.register_helper("time_since", Box::new(time_since_helper));

    let handlebars = Rc::new(handlebars);

    vec![
        index_file(handlebars.clone(), photographs),
        Box::new(HandlebarsFile::new(handlebars.clone(), "about.html", "text/html", Map::new())),
        Box::new(HandlebarsFile::new(handlebars.clone(), "gear.html", "text/html", Map::new())),

        Box::new(HandlebarsFile::new(handlebars.clone(), "assets/site.css", "text/css", Map::new())),
    ]
}

fn index_file(handlebars: Rc<Handlebars>, photographs: Vec<PhotographModel>) -> Box<dyn SiteFile> {
    let mut index = Map::new();
    index.insert("photographs".into(), to_json(photographs));

    Box::new(HandlebarsFile::new(handlebars, "index.html", "text/html", index))
}

handlebars_helper!(time_since_helper: |y: u64, m: u64, d: u64| { time_since(y as i32, m as u32, d as u32) });

fn time_since(y: i32, m: u32, d: u32) -> String {
    let now = chrono::Utc::today();
    let date = chrono::Utc.ymd(y, m, d);

    let diff = now - date;

    let years: f64 = diff.num_days() as f64 / 365.25f64;
    let months = years * 12f64;

    if months <  23.5f64 {
        format!("{:.0} month{}", months, if months as i32 == 1 { "" } else { "s" })
    } else {
        format!("{:.1} year{}", years, if years as i32 == 1 { "" } else { "s" })
    }
}