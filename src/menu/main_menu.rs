use std::io::{stdin};
use std::thread::sleep;
use std::time::Duration;
use rust_socketio::client::Client;
use crate::connect::connect::{create_game, upgrade_socket};
use crate::{connect, Token};

pub mod menu_state {
    pub static mut FIRST_START: bool = true;
}

/// entry for the main menu - can decide between single game and tournament
pub fn menu(mut socket: Client, jsontkn: &Token) {
    let mut decided_end = false;

    while !decided_end {
        println!("Was möchtest du tun?");
        println!("1: Einzelspiel");
        println!("2: Turnierspiel");
        println!("3: Beenden");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut _first_start = true;
        unsafe {
            _first_start = menu_state::FIRST_START;
        }

        match input.trim_end() {
            "1" => {
                if _first_start {
                    socket = upgrade_socket(jsontkn);
                    unsafe {
                        menu_state::FIRST_START = false;
                    }
                }
                socket = single_game(socket, jsontkn);
            },
            "2" => {
                if _first_start {
                    socket = upgrade_socket(jsontkn);
                    unsafe {
                        menu_state::FIRST_START = false;
                    }
                }
                socket = tournament_game(socket);
            },
            "3" => {
                decided_end = true;
                println!("Tschö");
            },
            _ =>
                println!("Ungültige eingabe, versuche es noch einmal.\n\n")
        }
        _ = &socket;
    }
}

/// single game menu - let's player decide if game should wait for invite or create game
fn single_game(mut socket: Client, jsontkn: &Token) -> Client{
    let mut correct_input = false;

    while !correct_input {
        println!("started single game!");

        println!("Was möchtest du tun?");
        println!("1: Auf Einladung warten");
        println!("2: Spiel erstellen");
        println!("3: Zurück");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();


        match input.trim_end() {
            "1" => {
                println!("Ab hier läufts automatisch.");
                unsafe {
                    crate::connect::events::current_game::SINGLE_GAME = true;
                    crate::connect::events::current_game::WAIT_FOR_INVITE = true;
                }
                loop {
                    sleep(Duration::from_secs(1));
                    unsafe{
                        if crate::connect::events::current_game::FINISHED{
                            break;
                        }
                    }
                }
                unsafe{
                    connect::events::current_game::FINISHED = false;
                }
            }

            "2" => {
                unsafe {
                    crate::connect::events::current_game::SINGLE_GAME = true;
                }
                socket = single_game_create(socket, jsontkn);
                'game: loop {
                    sleep(Duration::from_secs(1));
                    unsafe{
                        if crate::connect::events::current_game::FINISHED{
                            break 'game;
                        }
                    }
                    // println!("single game created loop");
                }
            },

            "3" => correct_input = true,

            _ => println!("Ungültige eingabe, versuche es noch einmal.\n\n")
        }
    }

    return socket;
}

/// internal function to create game
fn single_game_create(socket: Client, jsontkn: &Token) -> Client {
    create_game(jsontkn, Some(false), Some(false), Some(false));
    return socket;
}

/// internal function to wait for tournament invite
fn tournament_game(socket: Client) -> Client {
    println!("Turniermodus gestartet.\nWarte auf Einladung...");
    'tourney: loop {
        sleep(Duration::from_secs(2));
        unsafe{
            if connect::events::current_game::TOURNEY_FINISHED {
                break 'tourney;
            }
        }
    }
    unsafe{
        connect::events::current_game::TOURNEY_FINISHED = false;
    }
    return socket;
}

/// check if alternate account should be connected
pub fn load_alt() -> bool {
    println!("Soll der alternative Testclient verbunden werden? (yes/no)");

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    match input.trim_end() {
        "yes" | "y" | "ja" | "j" => return true,

        "no" | "n" | "nein" | _=> return false
    }

}
