use reqwest::{Client, Result};
use serde_json::Value;
use std::collections::HashMap;

pub async fn get(url: &str) -> Result<Value> {
    let response = Client::new().get(url).send().await?;
    let json = response.json().await?;
    Ok(json)
}

pub async fn post(url: &str, body: HashMap<&str, &str>) -> Result<Value> {
    let response = Client::new().post(url).json(&body).send().await?;
    let json = response.json().await?;
    Ok(json)
}

pub async fn put(url: &str, body: HashMap<&str, &str>) -> Result<Value> {
    let response = Client::new().put(url).json(&body).send().await?;
    let json = response.json().await?;
    Ok(json)
}

pub async fn delete(url: &str) -> Result<Value> {
    let response = Client::new().delete(url).send().await?;
    let json = response.json().await?;
    Ok(json)
}
