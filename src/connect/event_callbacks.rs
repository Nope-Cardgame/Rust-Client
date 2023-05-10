use rust_socketio::{Payload, RawClient};
use serde_json::json;


pub fn gameStateCallback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }

}



pub fn eliminatedCallback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }

}



pub fn gameInviteCallback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }

}



pub fn gameEndCallback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }

}



pub fn tournamentInviteCallback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }

}



pub fn tournemEndCallback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }

}