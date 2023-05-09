mod logic;
mod connect;
use dotenvy::dotenv;
use std::{time};
use std::thread::sleep;
use std::time::Duration;
use crate::connect::authenticate;
use rust_socketio::{Payload, ClientBuilder, RawClient, SocketBuilder};
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use futures_util::FutureExt;
use jsonwebtoken::crypto::sign;
use crate::connect::connect::{create_game, upgrade_socket};


#[derive( Debug, Deserialize, Serialize)]
pub struct Authtoken {
    pub auth: Value
}

#[derive( Debug, Deserialize, Serialize)]
pub struct Token {
    pub jsonwebtoken: String
}

fn main() {
    dotenv().ok();

    // create empty Token object holding the JWT
    let mut jsontkn : Token = Token {
        jsonwebtoken: "".to_string(),
    };

    // try to sign up to server
    let mut jsontoken = authenticate::sign_up();
    let test = authenticate::sign_in();

    // if signup failed try to log in 5 times
    let mut counter = 0;
    while jsontoken.is_err() && counter < 1{
        jsontoken = authenticate::sign_in();
        if jsontoken.is_err() {
            counter += 1;
            sleep(time::Duration::from_secs(5));
        }
    };

    // only proceed if JWT was received successfully
    if jsontoken.is_ok() {
        jsontkn.jsonwebtoken = jsontoken.unwrap();

        // connect::connect::get_user_connections(&jsontkn).await;

        let mut socket = upgrade_socket(&jsontkn);

        let connectedusers = connect::connect::get_user_connections(&jsontkn);

        create_game(&jsontkn, Some(false), Some(false), Some(false));

        let mut count = 0;
        loop {
            // socket.
            sleep(Duration::from_secs(1));
            count += 1;
            if count == 30 {
                break
            }
        }
        let disconnect = socket.disconnect();
    }
}
