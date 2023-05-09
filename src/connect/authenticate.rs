use reqwest;
use std::time::Duration;
use dotenvy;
use std::error::Error;
use serde_json::{Value, json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Body {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct Token {
    pub jsonwebtoken: String,
}

/// signs in user onto nope server
pub fn sign_in() -> Result<String, Box<dyn Error>>{
    let client = reqwest::blocking::Client::new();

    let login = Body {
        username: dotenvy::var("AUTH_USER").expect("no AUTH_USER in .env").into(),
        password: dotenvy::var("AUTH_PASS").expect("no AUTH_PASS in .env").into(),
    };

    let res = client
        .post(dotenvy::var("AUTH_URL").expect("error in auth: "))
        .json(&login)
        .send();

    let jsontoken = res.unwrap().json::<Token>();

    Ok(jsontoken?.jsonwebtoken)
}

/// signs up user with nope server
pub fn sign_up() -> Result<String, Box<dyn Error>>{

    let client = reqwest::blocking::Client::new();

    let login = Body {
        username: dotenvy::var("AUTH_USER").expect("no AUTH_USER in .env").into(),
        password: dotenvy::var("AUTH_PASS").expect("no AUTH_PASS in .env").into(),
    };

    let res = client
        .post(dotenvy::var("SIGN_URL").expect("error in auth: "))
        .json(&login)
        .send();

    let jsontoken = res.unwrap().json::<Token>();

    Ok(jsontoken?.jsonwebtoken)
}