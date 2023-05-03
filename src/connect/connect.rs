use reqwest;
use dotenvy;
use std::error::Error;
use serde::{Deserialize, Serialize};
use rust_socketio;
use serde_json::json;
use crate::Token;

#[derive(Debug, Deserialize, Serialize)]
struct Body {
    jsonwebtoken: String,
}

/// gets a json array with all currently connected users
/// uses simple HTTP GET request
pub async fn get_user_connections(token: &Token) -> Result<String, Box<dyn Error>>{
    let client = reqwest::Client::new();

    let res = client
        .get(dotenvy::var("API_URL").expect("error in auth: ") + "/userConnections")
        .header("Authorization", "Bearer ".to_owned() + &token.jsonwebtoken)
        .send()
        .await?;

    println!("response = {}", res.text().await?);

    Ok("".to_string())
}