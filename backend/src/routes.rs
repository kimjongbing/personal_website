use crate::models::Blog;
use crate::utils::*;
use comrak::{markdown_to_html, ComrakOptions};
use rocket::response::content;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use std::path::PathBuf;

use std::fs;
use std::path::Path;

#[get("/blogs")]
pub fn get_blog_articles() -> Json<Vec<Blog>> {
    let frontend_dir = get_frontend_directory();
    let blog_files_dir = frontend_dir.join("docs/blog_files");

    let blogs: Vec<Blog> = fs::read_dir(&blog_files_dir)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().ok().map_or(false, |ft| ft.is_file()))
                .filter_map(|entry| {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if file_name.ends_with(".md") {
                        Some(Blog {
                            name: file_name.clone(),
                            path: format!("docs/blog_files/{}", file_name),
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<Blog>>()
        })
        .unwrap_or_else(|err| {
            println!("Error reading blog files: {}", err);
            Vec::new()
        });

    Json(blogs)
}

#[get("/blog/<file..>")]
pub fn get_blog_article_content(file: PathBuf) -> content::Html<String> {
    let frontend_dir = get_frontend_directory();
    let path = frontend_dir.join(file);
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

#[get("/content.md")]
pub fn get_index_content() -> content::Html<String> {
    let frontend_dir = get_frontend_directory();
    let relative_path = Path::new("docs/index.md");
    let path = frontend_dir.join(relative_path);
    println!("Trying to read markdown from: {:?}", path);

    match fs::read_to_string(&path) {
        Ok(markdown) => {
            let mut html_content = markdown_to_html(&markdown, &ComrakOptions::default());

            html_content = replace_blog_placeholder_with_htmx(
                &html_content,
                "#placeholder_blog_files:",
                "hx-get=\"{}\" hx-swap=\"innerHTML\" hx-target=\"#content\"",
            );

            html_content = replace_placeholder_with_htmx(
                &html_content,
                "#placeholder_for_projects",
                "hx-get=\"projects.md\" hx-swap=\"innerHTML\" hx-target=\"#content\"",
            );
            html_content = replace_placeholder_with_htmx(
                &html_content,
                "#placeholder_for_blogs",
                "hx-get=\"blogs.md\" hx-swap=\"innerHTML\" hx-target=\"#content\"",
            );
            content::Html(html_content)
        }
        Err(err) => {
            println!("Error reading markdown file: {}", err);
            content::Html("Error reading markdown file".to_string())
        }
    }
}

#[get("/blogs.md")]
pub fn get_blogs_md_content() -> content::Html<String> {
    let frontend_dir = get_frontend_directory();
    let relative_path = Path::new("docs/blogs.md");
    let path = frontend_dir.join(relative_path);
    println!("Trying to read markdown from: {:?}", path);

    match fs::read_to_string(&path) {
        Ok(markdown) => {
            let mut html_content = markdown_to_html(&markdown, &ComrakOptions::default());
            html_content = replace_placeholder_with_htmx(
                &html_content,
                "#placeholder_for_index",
                "hx-get=\"content.md\" hx-swap=\"innerHTML\" hx-target=\"#content\"",
            );
            content::Html(html_content)
        }
        Err(err) => {
            println!("Error reading markdown file: {}", err);
            content::Html("Error reading markdown file".to_string())
        }
    }
}

#[get("/projects.md")]
pub fn get_projects_md_content() -> content::Html<String> {
    let frontend_dir = get_frontend_directory();
    let relative_path = Path::new("docs/projects.md");
    let path = frontend_dir.join(relative_path);
    println!("Trying to read markdown from: {:?}", path);

    match fs::read_to_string(&path) {
        Ok(markdown) => {
            let mut html_content = markdown_to_html(&markdown, &ComrakOptions::default());
            html_content = replace_placeholder_with_htmx(
                &html_content,
                "#placeholder_for_index",
                "hx-get=\"content.md\" hx-swap=\"innerHTML\" hx-target=\"#content\"",
            );
            content::Html(html_content)
        }
        Err(err) => {
            println!("Error reading markdown file: {}", err);
            content::Html("Error reading markdown file".to_string())
        }
    }
}

#[get("/<file..>", rank = 2)]
pub fn get_file_content(file: PathBuf) -> Option<NamedFile> {
    let frontend_dir = get_frontend_directory();
    let path = frontend_dir.join(file);
    println!("Trying to open file at: {:?}", path);
    NamedFile::open(path).ok()
}

#[get("/", rank = 1)]
pub fn get_index_page() -> Option<NamedFile> {
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
