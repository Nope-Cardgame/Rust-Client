mod logic;
mod connect;
use dotenvy::dotenv;
use std::{time};
use std::thread::sleep;
use std::time::Duration;
use crate::connect::authenticate;
use serde::{Deserialize, Serialize};
use crate::connect::connect::{create_game, upgrade_socket};

#[derive( Debug, Deserialize, Serialize)]
pub struct Token {
    pub jsonwebtoken: String
}

fn main() {
    // load all environment variables from .env file
    dotenv().ok();

    // create empty Token object holding the JWT
    let mut jsontkn : Token = Token {
        jsonwebtoken: "".to_string(),
    };

    // try to sign up to server
    let mut jsontoken = authenticate::sign_up();

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

        // create new socket.io socket
        let socket = upgrade_socket(&jsontkn);


        // test game creation request
        create_game(&jsontkn, Some(false), Some(false), Some(false));


        // loop to extend connection
        let mut count = 0;
        loop {

            // sleep(Duration::from_secs(1));
            // count += 1;
            // if count == 30 {
            //     break
            // }
        }

        // disconnect socket
        let _disconnect = socket.disconnect();
    }
}
