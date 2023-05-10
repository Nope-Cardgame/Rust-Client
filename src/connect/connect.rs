use dotenvy;
use std::error::Error;
use serde::{Deserialize, Serialize};
use rust_socketio;
use rust_socketio::{ClientBuilder, Payload, RawClient};
use serde_json::json;
use crate::connect::event_callbacks::{eliminated_callback, game_end_callback, game_invite_callback, game_state_callback, tournament_invite_callback, tournament_end_callback};
use crate::logic::game_objects::Game;
use crate::Token;

#[derive(Debug, Deserialize, Serialize)]
struct Body {
    jsonwebtoken: String,
}

pub type ConnectedPlayers = Vec<ConnPlayer>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnPlayer {
    pub username: String,
    pub socket_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CreateGameBody {
    pub noActionCards: bool,
    pub noWildCards: bool,
    pub oneMoreStartCards: bool,
    pub players: Vec<ConnPlayer>,
}

/// gets a json array with all currently connected users
/// uses simple HTTP GET request
pub fn get_user_connections(token: &Token) -> Result<ConnectedPlayers, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let res = client
        .get(dotenvy::var("API_URL").expect("error in auth: ") + "/userConnections")
        .header("Authorization", "Bearer ".to_owned() + &token.jsonwebtoken)
        .send();

    let response_string = r#res?.text()?;

    let players: ConnectedPlayers = serde_json::from_str(&response_string).unwrap();

    // println!("response = {:?}", res?.json::<ConnectedPlayers>());
    println!("response = {:?}", &players);

    Ok(players)
}

/// takes given JWT and tries to upgrade to websocket connection
pub fn upgrade_socket(token: &Token) -> rust_socketio::client::Client {

    //
    let socket = ClientBuilder::new(dotenvy::var("BASE_URL").expect("error in auth: "))
        .auth(json!({"token": token.jsonwebtoken}))
        .on("error", |err, _| eprintln!("Error: {:#?}", err))
        .on("gameState", |payload: Payload, socket: RawClient| game_state_callback(payload, socket))
        .on("eliminated", |payload: Payload, socket: RawClient| eliminated_callback(payload, socket))
        .on("gameInvite", |payload: Payload, socket: RawClient| game_invite_callback(payload, socket))
        .on("gameEnd", |payload: Payload, socket: RawClient| game_end_callback(payload, socket))
        .on("tournamentInvite", |payload: Payload, socket: RawClient| tournament_invite_callback(payload, socket))
        .on("tournamentEnd", |payload: Payload, socket: RawClient| tournament_end_callback(payload, socket))
        .connect()
        .expect("Connection failed");

    return socket;
}


pub fn create_game(token: &Token, no_action_cards: Option<bool>, no_wild_cards: Option<bool>, one_more_start_cards: Option<bool>) -> Game{

    let mut playing_players: Vec<ConnPlayer> = Vec::new();
    let connected_players = get_user_connections(token).unwrap();

    // get first connected Player that is not this client
    if connected_players.len() >= 2 {
        for player in &connected_players {
            if player.username == dotenvy::var("AUTH_USER").expect("error retrieving username from .env - create_game()") {
                playing_players.push(player.clone());
                break;
            }
        }
        if connected_players[0].username == dotenvy::var("AUTH_USER").expect("error retrieving username from .env - create_game()") {
            playing_players.push(connected_players[1].clone());
        }
        else {
            playing_players.push(connected_players[0].clone());
        }
    }
    // else {
    //     return;
    // }

    let body = CreateGameBody {
        noActionCards: no_action_cards.unwrap_or(false),
        noWildCards: no_wild_cards.unwrap_or(false),
        oneMoreStartCards: one_more_start_cards.unwrap_or(false),
        players: playing_players
    };

    let client = reqwest::blocking::Client::new();

    let res = client
        .post(dotenvy::var("API_URL").expect("error in auth: ") + "/game")
        .header("Authorization", "Bearer ".to_owned() + &token.jsonwebtoken)
        .json(&body)
        .send();

    let response_string = r#res.unwrap().text().unwrap();
    let game: Game = serde_json::from_str(&response_string).unwrap();

    println!("{:?}", &game);

    return game;
}
