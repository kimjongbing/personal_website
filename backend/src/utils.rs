use log::{debug, info};
use std::env;
use std::path::PathBuf;

pub fn get_frontend_directory() -> PathBuf {
    let current_dir = env::current_dir().expect("Failed to get the current working directory.");
    let mut frontend_dir = current_dir;
    frontend_dir.pop();
    let dir = frontend_dir.join("frontend");
    info!("Serving frontend files from: {:?}", dir);
    dir
}

pub fn replace_placeholder_with_htmx(content: &str, placeholder: &str, htmx_code: &str) -> String {
    debug!(
        "Attempting to replace {} with {} in content: {}",
        placeholder, htmx_code, content
    );
    let result = content.replace(
        &format!("<a href=\"{}\">", placeholder),
        &format!("<a href=\"#\" {}>", htmx_code),
    );
    debug!("Resulting content: {}", result);
    result
}

pub fn replace_blog_placeholder_with_htmx(
    html_content: &str,
    placeholder_prefix: &str,
    htmx_attributes: &str,
) -> String {
    debug!("replace_blog_placeholder_with_htmx called with html_content: {:?}, placeholder_prefix: {:?}, htmx_attributes: {:?}", html_content, placeholder_prefix, htmx_attributes);

    let placeholder = "#placeholder_blog_files:";
    let mut new_content = String::new();

    for line in html_content.lines() {
        debug!("Processing line: {:?}", line);
        if line.contains(placeholder_prefix) {
            let markdown_file = extract_filename_from_placeholder(line, placeholder).to_string();
            debug!(
                "Found placeholder_prefix in line. Extracted markdown_file: {:?}",
                markdown_file
            );
            let new_line = create_new_line(line, &markdown_file, htmx_attributes);
            debug!("Created new line: {:?}", new_line);
            new_content.push_str(&new_line);
        } else {
            new_content.push_str(line);
        }
        new_content.push('\n');
    }

    debug!(
        "replace_blog_placeholder_with_htmx returning: {:?}",
        new_content
    );

    new_content
}

fn extract_filename_from_placeholder<'a>(line: &'a str, placeholder: &str) -> &'a str {
    let start = line.find('"').unwrap() + 1;
    let end = line.rfind('"').unwrap();
    let prefix_length = placeholder.len();
    &line[(start + prefix_length)..end]
}

fn create_new_line(line: &str, markdown_file: &str, htmx_attributes: &str) -> String {
    debug!(
        "create_new_line called with line: {:?}, markdown_file: {:?}, htmx_attributes: {:?}",
        line, markdown_file, htmx_attributes
    );

    let new_line = format_replacement_line(markdown_file, htmx_attributes);
    replace_placeholder_with_new_line(line, markdown_file, &new_line)
}

fn format_replacement_line(markdown_file: &str, htmx_attributes: &str) -> String {
    debug!(
        "format_replacement_line called with markdown_file: {:?}, htmx_attributes: {:?}",
        markdown_file, htmx_attributes
    );

    let new_line = format!(
        "<a href=\"#\" hx-get=\"docs/blog_files/{}\" hx-swap=\"innerHTML\" hx-target=\"#content\" {}>",
        markdown_file, htmx_attributes
    );

    debug!("format_replacement_line returning: {:?}", new_line);

    new_line
}

fn replace_placeholder_with_new_line(line: &str, markdown_file: &str, new_line: &str) -> String {
    debug!("replace_placeholder_with_new_line called with line: {:?}, markdown_file: {:?}, new_line: {:?}", line, markdown_file, new_line);

    let new_content = line.replace(
        &format!("<a href=\"#placeholder_blog_files:{}\">", markdown_file),
        &new_line,
    );

    debug!(
        "replace_placeholder_with_new_line returning: {:?}",
        new_content
    );

    new_content
}
