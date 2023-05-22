use std::io::{stdin};
use std::thread::sleep;
use std::time::Duration;
use rust_socketio::client::Client;
use crate::connect::connect::{create_game, upgrade_socket};
use crate::connect::events::eliminated_callback;
use crate::Token;

pub fn menu(mut socket: Client, mut jsontkn: &Token) {
    let mut decided_end = false;

    while !decided_end {
        println!("Was möchtest du tun?");
        println!("1: Einzelspiel");
        println!("2: Turnierspiel");
        println!("3: Beenden");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match input.trim_end() {
            "1" => {
                socket = upgrade_socket(jsontkn);
                socket = single_game(socket, jsontkn);
            },
            "2" => {
                socket = upgrade_socket(jsontkn);
                socket = tournament_game(socket);
            },
            "3" => {
                decided_end = true;
                println!("Tschö");
            },
            _ =>
                println!("Ungültige eingabe, versuche es noch einmal.\n\n")
        }
    }
}

fn single_game(mut socket: Client, jsontkn: &Token) -> Client{
    println!("started single game!");

    println!("Was möchtest du tun?");
    println!("1: Auf Einladung warten");
    println!("2: Spiel erstellen");
    println!("3: Zurück");

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut correct_input = false;

    while !correct_input {
        match input.trim_end() {
            "1" => {
                println!("Ab hier läufts automatisch.");
                loop {
                    sleep(Duration::from_secs(1));
                }
            }

            "2" => {
                socket = single_game_create(socket, jsontkn);
                loop {
                    sleep(Duration::from_secs(1));
                }
            },

            "3" => correct_input = true,

            _ => println!("Ungültige eingabe, versuche es noch einmal.\n\n")
        }
    }



    return socket;
}

fn single_game_create(socket: Client, jsontkn: &Token) -> Client {
    create_game(jsontkn, Some(false), Some(false), Some(false));
    return socket;
}

fn tournament_game(socket: Client) -> Client {
    println!("Turniermodus gestartet.\nWarte auf Einladung...");
    loop {

    }

    return socket;
}