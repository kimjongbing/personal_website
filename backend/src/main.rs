#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::response::content;
use rocket::response::NamedFile;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use comrak::{markdown_to_html, ComrakOptions};

fn get_frontend_directory() -> PathBuf {
    let current_dir = env::current_dir().expect("Failed to get the current working directory.");
    let mut frontend_dir = current_dir;
    frontend_dir.pop();
    frontend_dir.join("frontend")
}

#[get("/content.md")]
fn content_md() -> content::Html<String> {
    let frontend_dir = get_frontend_directory();
    let relative_path = Path::new("docs/index.md");
    let path = frontend_dir.join(relative_path);
    println!("Trying to read markdown from: {:?}", path);

    match fs::read_to_string(&path) {
        Ok(markdown) => {
            let html_content = markdown_to_html(&markdown, &ComrakOptions::default());
            content::Html(html_content)
        }
        Err(err) => {
            println!("Error reading markdown file: {}", err);
            content::Html("Error reading markdown file".to_string())
        }
    }
}

#[get("/blogs.md")]
fn blogs_md() -> content::Html<String> {
    let frontend_dir = get_frontend_directory();
    let relative_path = Path::new("docs/blogs.md");
    let path = frontend_dir.join(relative_path);
    println!("Trying to read markdown from: {:?}", path);

    match fs::read_to_string(&path) {
        Ok(markdown) => {
            let html_content = markdown_to_html(&markdown, &ComrakOptions::default());
            content::Html(html_content)
        }
        Err(err) => {
            println!("Error reading markdown file: {}", err);
            content::Html("Error reading markdown file".to_string())
        }
    }
}

#[get("/projects.md")]
fn projects_md() -> content::Html<String> {
    let frontend_dir = get_frontend_directory();
    let relative_path = Path::new("docs/projects.md");
    let path = frontend_dir.join(relative_path);
    println!("Trying to read markdown from: {:?}", path);

    match fs::read_to_string(&path) {
        Ok(markdown) => {
            let html_content = markdown_to_html(&markdown, &ComrakOptions::default());
            content::Html(html_content)
        }
        Err(err) => {
            println!("Error reading markdown file: {}", err);
            content::Html("Error reading markdown file".to_string())
        }
    }
}

#[get("/<file..>", rank = 2)]
fn files(file: PathBuf) -> Option<NamedFile> {
    let frontend_dir = get_frontend_directory();
    let path = frontend_dir.join(file);
    println!("Trying to open file at: {:?}", path);
    NamedFile::open(path).ok()
}

#[get("/", rank = 1)]
fn index() -> Option<NamedFile> {
    let frontend_dir = get_frontend_directory();
    let path = frontend_dir.join("index.html");
    println!("Trying to open index at: {:?}", path);

    match NamedFile::open(path) {
        Ok(file) => Some(file),
        Err(err) => {
            println!("Error opening file: {}", err);
            None
        }
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files, content_md, blogs_md, projects_md])
        .launch();
}