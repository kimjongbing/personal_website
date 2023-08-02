use comrak::{markdown_to_html, ComrakOptions};
use log::error;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct Blog {
    pub name: String,
    pub path: String,
}

pub struct Content {
    pub path: PathBuf,
    pub content: String,
}

impl Content {
    pub fn new(path: PathBuf) -> Self {
        let markdown = fs::read_to_string(&path).unwrap_or_else(|err| {
            error!("Error reading markdown file: {}", err);
            "Error reading markdown file".to_string()
        });

        let content = markdown_to_html(&markdown, &ComrakOptions::default());
        log::debug!("Converted Markdown to HTML: {}", content);
        Self { path, content }
    }
}
