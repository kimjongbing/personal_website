#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

pub mod models;
pub mod routes;
pub mod utils;

use log::Level;
use simple_logger::SimpleLogger;

use crate::routes::*;

fn main() {
    SimpleLogger::new()
        .with_level(Level::Debug.to_level_filter())
        .init()
        .unwrap();
    rocket::ignite()
        .mount(
            "/",
            routes![
                get_index_page,
                get_file_content,
                get_index_content,
                get_blogs_md_content,
                get_projects_md_content,
                get_blog_articles,
                get_blog_article_content
            ],
        )
        .launch();
}
