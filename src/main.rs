use std::env;
use std::fs;
use std::process;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;

fn main() {
    // Replace this with your actual access token
    let access_token = "TOKEN GOES HERE";

    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <recipient_email> <file_path> <message_text>", args[0]);
        process::exit(1);
    }

    let recipient_email = &args[1];
    let file_path = &args[2];
    let message_text = &args[3];

    // Check if the file exists
    if !fs::metadata(file_path).is_ok() {
        eprintln!("Error: File not found at '{}'", file_path);
        process::exit(1);
    }

    // Get the recipient's person ID
    let person_id = match get_person_id(access_token, recipient_email) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    // Send the message with the file
    match send_message_with_file(access_token, &person_id, message_text, file_path) {
        Ok(_) => println!("File sent to {} successfully.", recipient_email),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn get_person_id(access_token: &str, email: &str) -> Result<String, String> {
    let client = Client::new();
    let url = format!("https://webexapis.com/v1/people?email={}", email);
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let resp = client
        .get(&url)
        .headers(headers)
        .send()
        .map_err(|e| format!("Failed to send GET request: {}", e))?;

    let status = resp.status();
    let text = resp
        .text()
        .map_err(|e| format!("Failed to read response text: {}", e))?;

    if !status.is_success() {
        return Err(format!("GET request failed with status {}: {}", status, text));
    }

    let json: Value =
        serde_json::from_str(&text).map_err(|e| format!("Failed to parse JSON: {}", e))?;
    let items = json["items"]
        .as_array()
        .ok_or("No 'items' field in response")?;
    if items.is_empty() {
        return Err(format!("No person found with email: {}", email));
    }
    let person_id = items[0]["id"]
        .as_str()
        .ok_or("No 'id' in person data")?
        .to_string();
    Ok(person_id)
}

fn send_message_with_file(
    access_token: &str,
    person_id: &str,
    message_text: &str,
    file_path: &str,
) -> Result<(), String> {
    let client = Client::new();
    let url = "https://webexapis.com/v1/messages";

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
    );

    let form = reqwest::blocking::multipart::Form::new()
        .text("toPersonId", person_id.to_string())
        .text("text", message_text.to_string())
        .file("files", file_path)
        .map_err(|e| format!("Failed to create multipart form: {}", e))?;

    let resp = client
        .post(url)
        .headers(headers)
        .multipart(form)
        .send()
        .map_err(|e| format!("Failed to send POST request: {}", e))?;

    let status = resp.status();
    if !status.is_success() {
        let text = resp
            .text()
            .unwrap_or_else(|_| "Failed to read response text".to_string());
        return Err(format!("POST request failed with status {}: {}", status, text));
    }

    Ok(())
}
