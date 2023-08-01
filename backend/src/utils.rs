use std::env;
use std::path::PathBuf;

pub fn get_frontend_directory() -> PathBuf {
    let current_dir = env::current_dir().expect("Failed to get the current working directory.");
    let mut frontend_dir = current_dir;
    frontend_dir.pop();
    frontend_dir.join("frontend")
}

pub fn replace_placeholder_with_htmx(content: &str, placeholder: &str, htmx_code: &str) -> String {
    println!(
        "Attempting to replace {} with {} in content: {}",
        placeholder, htmx_code, content
    );
    let result = content.replace(
        &format!("<a href=\"{}\">", placeholder),
        &format!("<a href=\"#\" {}>", htmx_code),
    );
    println!("Resulting content: {}", result);
    result
}

pub fn replace_blog_placeholder_with_htmx(
    html_content: &str,
    placeholder_prefix: &str,
    _htmx_attributes: &str, // this still doesnt do anything btw
) -> String {
    let placeholder = "#placeholder_blog_files:";
    let mut new_content = String::new();

    for line in html_content.lines() {
        if line.contains(placeholder_prefix) {
            let markdown_file = extract_filename_from_placeholder(line, placeholder).to_string();
            new_content.push_str(&create_new_line(line, &markdown_file));
        } else {
            new_content.push_str(line);
        }
        new_content.push('\n');
    }

    new_content
}

fn extract_filename_from_placeholder<'a>(line: &'a str, placeholder: &str) -> &'a str {
    let start = line.find('"').unwrap() + 1;
    let end = line.rfind('"').unwrap();
    let prefix_length = placeholder.len();
    &line[(start + prefix_length)..end]
}

fn create_new_line(line: &str, markdown_file: &str) -> String {
    let new_line = format_replacement_line(markdown_file);
    replace_placeholder_with_new_line(line, markdown_file, &new_line)
}

fn format_replacement_line(markdown_file: &str) -> String {
    format!(
        "<a href=\"#\" hx-get=\"docs/blog_files/{}\" hx-swap=\"innerHTML\" hx-target=\"#content\">",
        markdown_file
    )
}

fn replace_placeholder_with_new_line(line: &str, markdown_file: &str, new_line: &str) -> String {
    line.replace(
        &format!("<a href=\"#placeholder_blog_files:{}\"", markdown_file),
        &new_line,
    )
}
