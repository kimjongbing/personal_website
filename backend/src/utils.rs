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
    _htmx_attributes: &str, // doesnt do anything yet, dont remember why its here and i seemingly cant remove it or this breaks
) -> String {
    let mut new_content = String::new();

    for line in html_content.lines() {
        if line.contains(placeholder_prefix) {
            println!("Found placeholder: {}", line);
            // get the name of the markdown file
            let start = line.find('"').unwrap() + 1;
            let end = line.rfind('"').unwrap();
            let prefix_length = "#placeholder_blog_files:".len();
            let md_file = &line[(start + prefix_length)..end];

            let new_line = format!("<a href=\"#\" hx-get=\"docs/blog_files/{}\" hx-swap=\"innerHTML\" hx-target=\"#content\">", md_file);
            println!("New line: {}", new_line);

            new_content.push_str(&line.replace(
                &format!("<a href=\"#placeholder_blog_files:{}\"", md_file),
                &new_line,
            ));
        } else {
            // if the line doesn't contain the placeholder, just add it to the new content
            new_content.push_str(line);
        }
        new_content.push('\n');
    }

    new_content
}

// clean up this code so i dont need comments to explain it
