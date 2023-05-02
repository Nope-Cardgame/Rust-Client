mod logic;
mod connect;
use dotenvy::dotenv;
use std::{env, time};
use std::thread::sleep;
use serde::ser::Error;
use serde_json::json;
use crate::connect::authenticate::Token;
use crate::connect::authenticate;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut jsontoken = authenticate::sign_up().await;

    while jsontoken.is_err() {
        jsontoken = authenticate::sign_in().await;
        sleep(time::Duration::from_secs(5));
    }

    println!("{}", jsontoken.unwrap());
}
