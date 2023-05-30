use rust_socketio::RawClient;
use serde_json::json;
use crate::logic::game_objects::{Card, DiscardAction, Game, GamePlayer, NopeAction, TakeAction};

pub mod cards {
    use crate::logic::game_objects::Card;

    pub static mut CARDS: Vec<Card> = Vec::new();
    pub static mut TOOK_CARDS: bool = false;
}

/// this function plays a simple turn selecting the first available cards
/// it can only play rule-safe turns
pub unsafe fn basic_turn(game_state: &Game, socket: &RawClient) {
    static mut TOOK_CARDS: bool = false;
    cards::CARDS = [].to_vec();
    let players = game_state.clone().players.unwrap();
    let opponent = players.get(1).unwrap().clone();

    for player in  players {
        if player.username == dotenvy::var("AUTH_USER").expect("error retrieving username from .env - create_game()") {
            cards::CARDS = player.cards.unwrap();
        }
    }

    // get the first card of the discard pile and regard is as the deciding card
    let discard_pile = game_state.clone().discardPile.unwrap();
    let decider = discard_pile.get(0).unwrap();

    // match the decider type
    match decider.type_field.trim_end() {
        // search if hand has the right cards
        "number" => {
            let play_successful = number_card(&decider, &cards::CARDS, &opponent, socket);

            if !play_successful {
                // take cards if couldn't discard from hand
                if !cards::TOOK_CARDS {
                    cards::TOOK_CARDS = true;
                    take_cards(socket);
                }
                // nope if already took cards
                else {
                    TOOK_CARDS = false;
                    play_nope(socket);
                }

            }

        }

        // action cards section


        "reset" => {
            println!("Received reset card!");
        }
        "invisible" => {
            println!("Received invis card!");
        }
        "nominate" => {
            println!("Received nominate card!");
        }
        _ => {
            println!("invalid card type received from server!");
        }

    }
}

/// check if the card colour & amount is in hands
/// send cards if successful
fn number_card(decider: &Card, current_cards: &Vec<Card>, opponent: &GamePlayer, socket: &RawClient) -> bool {
    println!("Received number card!");
    for decider_color in decider.colors.as_ref().unwrap() {
        let mut possible_cards: Vec<Card> = [].to_vec();

        for card in current_cards {
            // give inner for iteration a label to be able to break only this iteration
            'color: for color in card.colors.as_ref().unwrap() {
                if color.to_string() == decider_color.to_string() {
                    possible_cards.push(card.clone());
                    break 'color;
                }
            }
        }

        if possible_cards.len() >= (decider.value.unwrap()) as usize {
            unsafe{
                cards::TOOK_CARDS = false;
            }
            play_cards(possible_cards[..(decider.value.unwrap()) as usize].to_vec(), socket, opponent);

            return true;
        }
    }
    return false;
}

/// internal function to send play cards event
fn play_cards(cards: Vec<Card>, socket: &RawClient, _opponent: &GamePlayer) {
    println!("playing {:?}", cards);
    let action_body = DiscardAction {
        type_field: "discard".to_string(),
        explanation: "playing first available cards".to_string(),
        cards: Option::from(cards),
    };

    let payload = json!(&action_body);
    println!("sending played cards: {}", &payload);

    socket.emit("playAction", payload).expect("error in sending played cards");
}

/// internal function to send nope event
fn play_nope(socket: &RawClient) {
    println!("no cards to play... nope!");
    let action_body = NopeAction {
        type_field: "nope".to_string(),
        explanation: "no cards to play".to_string(),
        player: None,
    };

    let payload = json!(&action_body);
    println!("sending nope: {}", &payload);

    unsafe{
        cards::TOOK_CARDS = false;
    }
    socket.emit("playAction", payload).expect("error in sending played cards");
}

/// send event to take cards
pub fn take_cards(socket: &RawClient) {
    println!("no cards to play, taking cards!");
    let action_body = TakeAction {
        type_field: "take".to_string(),
        explanation: "no cards to play".to_string(),
    };

    let payload = json!(&action_body);
    println!("sending takeCards: {}", &payload);

    socket.emit("playAction", payload).expect("error in sending takeCards");
}