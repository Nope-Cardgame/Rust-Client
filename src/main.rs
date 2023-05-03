mod logic;
mod connect;
use dotenvy::dotenv;
use std::{time};
use std::thread::sleep;
use crate::connect::authenticate;
use rust_socketio::{Payload, {asynchronous::{ClientBuilder, Client}}, RawClient};
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use futures_util::FutureExt;
use crate::connect::connect::get_user_connections;

#[derive( Debug, Deserialize, Serialize)]
pub struct Authtoken {
    pub auth: Value
}

#[derive( Debug, Deserialize, Serialize)]
pub struct Token {
    pub jsonwebtoken: String
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // create empty Token object holding the JWT
    let mut jsontkn : Token = Token {
        jsonwebtoken: "".to_string(),
    };

    // try to sign up to server
    let mut jsontoken = authenticate::sign_up().await;

    // if signup failed try to log in 5 times
    let mut counter = 0;
    while jsontoken.is_err() && counter < 5{
        jsontoken = authenticate::sign_in().await;
        if jsontoken.is_err() {
            counter += 1;
            sleep(time::Duration::from_secs(5));
        };
    };

    // only proceed if JWT was received successfully
    if jsontoken.is_ok() {
        jsontkn.jsonwebtoken = jsontoken.unwrap();



        // get_user_connections(&jsontkn).await;

        let callback = |payload: Payload, socket: Client| {
            println!("inside callback");
            async move {
                match payload {
                    Payload::String(str) => println!("Received: {}", str),
                    Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),

                }
                socket
                    .emit("test", json!({"got ack": true}))
                    .await
                    .expect("Server unreachable");
            }
                .boxed()
        };

        let mut authContent = "Bearer ".to_owned() + &jsontkn.jsonwebtoken;
        let mut socket = ClientBuilder::new(dotenvy::var("BASE_URL").expect("error in auth: "))
            // let socket = ClientBuilder::new("https://nopecardgame.requestcatcher.com/")
            .opening_header("Authorization", &*authContent)
            .on("connect", callback)
            .connect()
            .await
            .expect("Connection failed");

        socket.emit("auth", &*authContent);
    }



}
