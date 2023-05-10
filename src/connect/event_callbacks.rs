use rust_socketio::{Payload, RawClient};
use serde_json::json;


pub fn game_state_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("gameState received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }

}



pub fn eliminated_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("eliminated Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }

}



pub fn game_invite_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("gameInvite Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }

}



pub fn game_end_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("gameEnd Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }

}



pub fn tournament_invite_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("tournamentInvite Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }

}



pub fn tournament_end_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("tournamentEnd Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }

}