use crate::models::{Blog, Content};
use crate::utils::*;
use rocket::response::content;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use std::fs;
use std::path::{Path, PathBuf};
use log::{info, error, debug};

#[get("/blogs")]
pub fn get_blog_articles() -> Json<Vec<Blog>> {
    info!("get_blog_articles has been called");
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
            error!("Error reading blog files: {}", err);
            Vec::new()
        });

    info!("get_blog_articles is about to return");
    Json(blogs)
}

#[get("/docs/blog_files/<file..>", rank = 1)]
pub fn get_blog_article_content(file: PathBuf) -> content::Html<String> {
    info!("get_blog_article_content called with file: {:?}", file);
    let frontend_dir = get_frontend_directory();
    let path = frontend_dir.join("docs").join("blog_files").join(file);
    debug!("Full path to the file: {:?}", path);
    let mut content = Content::new(path);

    content.content = replace_placeholder_with_htmx(
        &content.content,
        "#placeholder_for_index",
        "hx-get=\"content.md\" hx-swap=\"innerHTML\" hx-target=\"#content\"",
    );

    content.content = replace_placeholder_with_htmx(
        &content.content,
        "#placeholder_for_blogs",
        "hx-get=\"blogs.md\" hx-swap=\"innerHTML\" hx-target=\"#content\"",
    );

    info!("get_blog_article_content is about to return");
    debug!("get_blog_article_content: {}", &content.content);
    content::Html(content.content)
}

#[get("/content.md")]
pub fn get_index_content() -> content::Html<String> {
    info!("get_index_content has been called");
    let frontend_dir = get_frontend_directory();
    let relative_path = Path::new("docs/index.md");
    let path = frontend_dir.join(relative_path);
    let mut content = Content::new(path);

    content.content = replace_blog_placeholder_with_htmx(
        &content.content,
        "#placeholder_blog_files:",
        "hx-get=\"{}\" hx-swap=\"innerHTML\" hx-target=\"#content\"",
    );

    content.content = replace_placeholder_with_htmx(
        &content.content,
        "#placeholder_for_projects",
        "hx-get=\"projects.md\" hx-swap=\"innerHTML\" hx-target=\"#content\"",
    );

    content.content = replace_placeholder_with_htmx(
        &content.content,
        "#placeholder_for_blogs",
        "hx-get=\"blogs.md\" hx-swap=\"innerHTML\" hx-target=\"#content\"",
    );

    info!("get_index_content is about to return");
    debug!("get_index_content: {}", &content.content);
    content::Html(content.content)
}

#[get("/blogs.md")]
pub fn get_blogs_md_content() -> content::Html<String> {
    info!("get_blogs_md_content has been called");
    let frontend_dir = get_frontend_directory();
    let relative_path = Path::new("docs/blogs.md");
    let path = frontend_dir.join(relative_path);
    let mut content = Content::new(path);

    content.content = replace_placeholder_with_htmx(
        &content.content,
        "#placeholder_for_index",
        "hx-get=\"content.md\" hx-swap=\"innerHTML\" hx-target=\"#content\"",
    );

    content.content = replace_blog_placeholder_with_htmx(
        &content.content,
        "#placeholder_blog_files:",
        "hx-get=\"{}\" hx-swap=\"innerHTML\" hx-target=\"#content\"",
    );

    info!("get_blogs_md_content is about to return");
    debug!("get_index_content: {}", &content.content);
    content::Html(content.content)
}

#[get("/projects.md")]
pub fn get_projects_md_content() -> content::Html<String> {
    info!("get_projects_md_content has been called");
    let frontend_dir = get_frontend_directory();
    let relative_path = Path::new("docs/projects.md");
    let path = frontend_dir.join(relative_path);
    let mut content = Content::new(path);

    content.content = replace_placeholder_with_htmx(
        &content.content,
        "#placeholder_for_index",
        "hx-get=\"content.md\" hx-swap=\"innerHTML\" hx-target=\"#content\"",
    );

    info!("get_projects_md_content is about to return");
    debug!("get_projects_md_content: {}", &content.content);
    content::Html(content.content)
}

#[get("/files/<file..>")]
pub fn get_file_content(file: PathBuf) -> Option<NamedFile> {
    info!("get_file_content has been called with file: {:?}", file);
    let frontend_dir = get_frontend_directory();
    let path = frontend_dir.join(file);
    debug!("Full path to the file: {:?}", path);
    NamedFile::open(path).ok()
}

#[get("/")]
pub fn get_index_page() -> Option<NamedFile> {
    info!("index has been called");
    let frontend_dir = get_frontend_directory();
    let path = frontend_dir.join("index.html");
    debug!("Full path to the file: {:?}", path);
    NamedFile::open(path).ok()
}
