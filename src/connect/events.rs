use std::io::stdin;
use std::ops::Deref;
use rust_socketio::{Event, Payload, RawClient, Socket};
use serde_json::json;
use crate::logic::game_objects::{Eliminated, Game, Ready, Tournament};
use crate::connect::connect::CurrentGame;
use crate::logic::turn::basic_turn;


pub mod current_game {
    pub static mut ONGOING: bool = false;
    pub static mut FINISHED: bool = false;
    pub static mut SINGLE_GAME: bool = false;
    pub static mut TOURNEY_FINISHED: bool = false;
    pub static mut WAIT_FOR_INVITE: bool = false;
}


pub fn socket_connect(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => println!("gameState received: {}", str),
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }
}

/// callback for gameState event, no current functionality
pub fn game_state_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => {
            println!("gameState received: {}", str);
            let game_state: Game = serde_json::from_str(&str).unwrap();
            if game_state.currentPlayer.as_ref().unwrap().username == dotenvy::var("AUTH_USER").expect("error retrieving username from .env - create_game()"){
                unsafe{
                    basic_turn(&game_state, &socket);
                }
            }
        },
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }
}


/// callback for eliminated event, no current functionality
pub fn eliminated_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => {
            println!("eliminated Received: {}", str);
            let eliminated_result = serde_json::from_str(&str);
            if eliminated_result.is_ok(){
                let eliminated: Eliminated = eliminated_result.unwrap();
                if !eliminated.disqualified {
                    println!("You sadly lost the round. No more cards on your hand!");
                }
            }

        },
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }
}


/// callback for gameInvite event, no current functionality
pub fn game_invite_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => {
            println!("gameInvite Received: {}", str);
            let game: Game = serde_json::from_str(&str).unwrap();
            let players = game.players.unwrap().clone();
            let opponent = players.get(0).unwrap().clone();

            let mut single_game: bool = true;
            let mut wait_for_invite: bool = false;
            unsafe {
                single_game = crate::connect::events::current_game::SINGLE_GAME;
                wait_for_invite = crate::connect::events::current_game::WAIT_FOR_INVITE;
            }

            if single_game && wait_for_invite{
                println!("Game invite received against {}. Do you want to accept? (yes/no)", opponent.username);

                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();

                match input.trim_end() {
                    "yes" | "y" | "ja" | "j" => {}
                    "no" | "n" | "nein" => {}
                    _ => {}
                }
                unsafe {
                    if current_game::ONGOING == false {
                        current_game::FINISHED = false;
                        current_game::ONGOING = true;
                        ready(socket, "game".to_string(), game.id.expect("game id not available").to_string());
                    }
                }
            }
            else if single_game && !wait_for_invite {
                println!("Game invite received against {}.", opponent.username);
                unsafe {
                    if current_game::ONGOING == false {
                        current_game::FINISHED = false;
                        current_game::ONGOING = true;
                        ready(socket, "game".to_string(), game.id.expect("game id not available").to_string());
                    }
                }
            }
            else {
                println!("Tournament game invite received. Accepting.");
                ready(socket, "game".to_string(), game.id.expect("game id not available").to_string());
            }
        }
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }
}


/// callback for gameEnd event, no current functionality
pub fn game_end_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => {
            println!("gameEnd Received: {}", str);


            unsafe{
                current_game::ONGOING = false;
                current_game::FINISHED = true;
            }
            let game_result = serde_json::from_str::<Game>(&str);

            println!("{:?}", game_result)
        },
        Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
    }
}


/// callback for tournamentInvite event, no current functionality
pub fn tournament_invite_callback(payload: Payload, socket: RawClient) {
    match payload {
        Payload::String(str) => {
            println!("tournamentInvite Received: {}", str);
            let game: Tournament = serde_json::from_str(&str).unwrap();
        },
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