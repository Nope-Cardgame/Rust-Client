use std::collections::HashMap;
use reqwest;
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


pub async fn sign_in() -> Result<String, Box<dyn Error>>{
    let client = reqwest::Client::new();

    let login = Body {
        username: dotenvy::var("AUTH_PASS").expect("no AUTH_PASS in .env").into(),
        password: dotenvy::var("AUTH_USER").expect("no AUTH_USER in .env").into(),
    };

    let res = client
        .post(dotenvy::var("AUTH_URL").expect("error in auth: "))
        .json(&login)
        .send()
        .await?;

    // println!("response = {:#?}", &res);
    let jsontoken = res.json::<Token>().await?;

    Ok(jsontoken.jsonwebtoken)
}

pub async fn sign_up() -> Result<String, Box<dyn Error>>{

    let client = reqwest::Client::new();

    let login = Body {
        username: dotenvy::var("AUTH_PASS").expect("no AUTH_PASS in .env").into(),
        password: dotenvy::var("AUTH_USER").expect("no AUTH_USER in .env").into(),
    };

    let res = client
        .post(dotenvy::var("SIGN_URL").expect("error in auth: "))
        .json(&login)
        .send()
        .await?;

    // println!("response = {:#?}", res);

    let jsontoken = res.json::<Token>().await?;

    Ok(jsontoken.jsonwebtoken)

}