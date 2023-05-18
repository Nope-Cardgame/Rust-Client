use rust_socketio::{Payload, RawClient};
use serde_json::json;
use crate::connect::connect::CreateGameBody;
use crate::logic::game_objects::{Game, Ready};

/// callback for gameState event, no current functionality
pub fn game_state_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("gameState received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }
}


/// callback for eliminated event, no current functionality
pub fn eliminated_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("eliminated Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }
}


/// callback for gameInvite event, no current functionality
pub fn game_invite_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => {
            println!("gameInvite Received: {}", str);
            let game: Game = serde_json::from_str(&str).unwrap();
            ready(socket, "game".to_string(), game.id.expect("game id not available").to_string())
        }
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }
}


/// callback for gameEnd event, no current functionality
pub fn game_end_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("gameEnd Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }
}


/// callback for tournamentInvite event, no current functionality
pub fn tournament_invite_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("tournamentInvite Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }
}


/// callback for tournamentEnd event, no current functionality
pub fn tournament_end_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("tournamentEnd Received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }
}


pub fn ready(socket: RawClient, game_type: String, invite_id: String) {
    let rdy = Ready {
        type_field: game_type,
        accept: true,
        inviteId: invite_id,
    };

    let payload = json!(&rdy);
    println!("sending ready: {}", &payload);

    socket.emit("ready", payload).expect("error in sending ready");
}