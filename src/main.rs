mod logic;
mod connect;
mod menu;

use dotenvy::dotenv;
use std::time;
use std::thread::sleep;
use rust_socketio::client::Client;
use rust_socketio::ClientBuilder;
use crate::connect::authenticate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub jsonwebtoken: String,
}

fn main() {
    // load all environment variables from .env file
    if menu::main_menu::load_alt() {
        dotenvy::from_filename(".alt.env").ok();
    }
    else {
        dotenv().ok();
    }

    // create empty Token object holding the JWT
    let mut jsontkn: Token = Token {
        jsonwebtoken: "".to_string(),
    };

    let socket: Client = ClientBuilder::new(dotenvy::var("BASE_URL").expect("error in auth: "))
        .connect()
        .expect("dummy client ok");

    // try to sign up to server
    let mut jsontoken = authenticate::sign_up();

    // if signup failed try to log in 5 times
    let mut counter = 0;
    while jsontoken.is_err() && counter < 1 {
        jsontoken = authenticate::sign_in();
        if jsontoken.is_err() {
            counter += 1;
            sleep(time::Duration::from_secs(5));
        }
    };

    // only proceed if JWT was received successfully
    if jsontoken.is_ok() {
        jsontkn.jsonwebtoken = jsontoken.unwrap();

        menu::main_menu::menu(socket, &jsontkn);

    }
}
