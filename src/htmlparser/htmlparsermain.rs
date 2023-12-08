use std::fs;
use std::io;

struct HtmlData {
    // Define the fields of the struct to store the HTML information
    // For example:
    title: String,
    body: String,
    // Add more fields as needed
}

fn parse_html(html: &str) -> HtmlData {
    // Parse the HTML and extract the necessary information
    // For example:
    let title = extract_title(html);
    let body = extract_body(html);

    // Create a new instance of the HtmlData struct and populate it with the extracted information
    HtmlData {
        title,
        body,
        // Initialize other fields as needed
    }
}

fn extract_title(html: &str) -> String {
    // Extract the title from the HTML
    // Implement your logic here
    // For example:
    "Example Title".to_string()
}

fn extract_body(html: &str) -> String {
    // Extract the body from the HTML
    // Implement your logic here
    // For example:
    "Example Body".to_string()
}


fn open_html_file(file_path: &str) -> Result<HtmlData, io::Error> {
    let html = fs::read_to_string(file_path)?;
    let html_data = parse_html(&html);
    Ok(html_data)
}
