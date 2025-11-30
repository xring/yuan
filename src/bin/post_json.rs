use anyhow::{anyhow, Context, Result};
use chrono::Local;
use std::env;
use std::fs;
use std::io::Write;

fn main() -> Result<()> {
    // Get file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(anyhow!("Usage: {} <file_path>", args[0]));
    }
    let file_path = &args[1];

    // Read the file
    let content = fs::read_to_string(file_path)
        .context(format!("Failed to read file: {}", file_path))?;

    // Parse the file content
    let mut lines = content.lines();
    let url = lines
        .next()
        .ok_or_else(|| anyhow!("File is empty, expected URL on first line"))?
        .trim();

    // Skip the empty line
    let second_line = lines.next().unwrap_or("");
    if !second_line.trim().is_empty() {
        return Err(anyhow!("Second line should be empty"));
    }

    // Collect the remaining lines as JSON
    let json_content: String = lines.collect::<Vec<&str>>().join("\n");
    let json_content = json_content.trim();

    if json_content.is_empty() {
        return Err(anyhow!("No JSON content found after the second line"));
    }

    // Validate JSON
    let _json_value: serde_json::Value = serde_json::from_str(json_content)
        .context("Failed to parse JSON content")?;

    // Create HTTP client
    let client = reqwest::blocking::Client::builder()
        .build()
        .context("Failed to create HTTP client")?;

    // Prepare the request
    let request = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(json_content.to_string())
        .build()
        .context("Failed to build HTTP request")?;

    // Generate output filename with timestamp
    let timestamp = Local::now().format("%Y%m%d-%H%M%S");
    let output_filename = format!("yuan-post-json-{}.txt", timestamp);

    // Prepare log content (curl -v format)
    let mut log_content = String::new();

    // Log request
    log_content.push_str(&format!("> POST {} HTTP/1.1\n", request.url()));
    log_content.push_str(&format!("> Host: {}\n", request.url().host_str().unwrap_or("")));

    // Log request headers
    for (name, value) in request.headers() {
        log_content.push_str(&format!("> {}: {}\n", name, value.to_str().unwrap_or("<binary>")));
    }
    log_content.push_str(">\n");

    // Log request body
    log_content.push_str(&format!("{}\n\n", json_content));

    // Send the request
    let response = client
        .execute(request)
        .context("Failed to send HTTP request")?;

    // Log response status
    log_content.push_str(&format!("< HTTP/1.1 {} {}\n",
        response.status().as_u16(),
        response.status().canonical_reason().unwrap_or("")));

    // Log response headers
    for (name, value) in response.headers() {
        log_content.push_str(&format!("< {}: {}\n", name, value.to_str().unwrap_or("<binary>")));
    }
    log_content.push_str("<\n");

    // Get response body
    let response_text = response.text().context("Failed to read response body")?;

    // Try to parse response as JSON
    if let Ok(response_json) = serde_json::from_str::<serde_json::Value>(&response_text) {
        // If it's valid JSON, log it (pretty printed)
        log_content.push_str(&serde_json::to_string_pretty(&response_json)
            .unwrap_or(response_text.clone()));
        log_content.push('\n');
    }
    // If not JSON, don't log the response body (as per requirements)

    // Write to file
    let mut file = fs::File::create(&output_filename)
        .context(format!("Failed to create output file: {}", output_filename))?;
    file.write_all(log_content.as_bytes())
        .context("Failed to write to output file")?;

    println!("Request and response logged to: {}", output_filename);

    Ok(())
}
