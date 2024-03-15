use std::io;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::json;


#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    // Define the structure of the API response data
    // This can vary depending on your API's response format
    // For simplicity, let's assume it's JSON
}

async fn get(url: &str) -> Result<Response, reqwest::Error> {
    // Implement GET method
    Client::new().get(url).send().await
}

async fn post(url: &str, body: HashMap<&str, &str>) -> Result<Response, reqwest::Error> {
    // Serialize the body to JSON
    let body_json = json!(body);

    // Send POST request with JSON body
    Client::new()
        .post(url)
        .header("Content-Type", "application/json")
        .body(body_json.to_string())
        .send()
        .await
}

async fn put(url: &str, body: HashMap<&str, &str>) -> Result<Response, reqwest::Error> {
    // Serialize the body to JSON
    let body_json = json!(body);

    // Send PUT request with JSON body
    Client::new()
        .put(url)
        .header("Content-Type", "application/json")
        .body(body_json.to_string())
        .send()
        .await
}

async fn delete(url: &str) -> Result<Response, reqwest::Error> {
    // Implement DELETE method
    Client::new().delete(url).send().await
}

async fn handle_response(response: Response) -> Result<(), reqwest::Error> {
    // Process and display the response
    let status = response.status();
    let body = response.text().await?;

    println!("Status: {}", status);
    println!("Body: {}", body);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("REST CLI Tester");

    loop {
        println!("Enter your choice:");
        println!("1. GET");
        println!("2. POST");
        println!("3. PUT");
        println!("4. DELETE");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim().parse() {
            Ok(1) => {
                println!("Enter URL:");
                let mut url = String::new();
                io::stdin().read_line(&mut url)?;
                let response = get(&url.trim()).await?;
                handle_response(response).await?;
            }
            Ok(2) => {
                println!("Enter URL:");
                let mut url = String::new();
                io::stdin().read_line(&mut url)?;

                println!("Enter JSON Body:");
                let mut body_str = String::new();
                io::stdin().read_line(&mut body_str)?;

                let body: HashMap<&str, &str> = serde_json::from_str(&body_str)?;
                let response = post(&url.trim(), body).await?;
                handle_response(response).await?;
            }
            Ok(3) => {
                println!("Enter URL:");
                let mut url = String::new();
                io::stdin().read_line(&mut url)?;

                println!("Enter JSON Body:");
                let mut body_str = String::new();
                io::stdin().read_line(&mut body_str)?;

                let body: HashMap<&str, &str> = serde_json::from_str(&body_str)?;
                let response = put(&url.trim(), body).await?;
                handle_response(response).await?;
            }
            Ok(4) => {
                println!("Enter URL:");
                let mut url = String::new();
                io::stdin().read_line(&mut url)?;
                let response = delete(&url.trim()).await?;
                handle_response(response).await?;
            }
            Ok(5) => break,
            _ => println!("Invalid choice!"),
        }
    }

    Ok(())
}