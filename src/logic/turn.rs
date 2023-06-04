use rust_socketio::RawClient;
use serde_json::json;
use crate::logic::game_objects::{Card, DiscardAction, Game, GamePlayer, NominateAction, NopeAction, TakeAction};

pub mod cards {
    use crate::logic::game_objects::Card;

    pub static mut CARDS: Vec<Card> = Vec::new();
    pub static mut TOOK_CARDS: bool = false;
}

/// this function plays a simple turn selecting the first available cards
/// it can only play rule-safe turns
pub unsafe fn ai_turn(game_state: &Game, socket: &RawClient) {
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
    let mut decider = discard_pile.get(0).unwrap();

    // match the decider type
    match decider.type_field.trim_end() {
        // search if hand has the right cards
        "number" => {
            discard_loop(decider, &opponent, socket);
        }

        // action cards section

        "reset" => {
            println!("Received reset card!");
            let mut play_first: Vec<Card> = [].to_vec();
            play_first.push(cards::CARDS[0].clone());
            discard_cards(play_first, socket, &opponent)
        }
        "invisible" => {
            println!("Received invis card!");
            if discard_pile.len() > 1 {
                decider = discard_pile.get(1).unwrap();
                discard_loop(decider, &opponent, socket);
            }
            else {
                discard_loop(decider, &opponent, socket);
            }

        }
        "nominate" => {
            println!("Received nominate card!");
        }
        _ => {
            println!("invalid card type received from server!");
        }

    }
}

fn discard_loop(decider: &Card, opponent: &GamePlayer, socket: &RawClient) {
    unsafe {
        let play_successful = number_card(&decider, &cards::CARDS, &opponent, socket);

        if !play_successful {
            // take cards if couldn't discard from hand
            if !cards::TOOK_CARDS {
                cards::TOOK_CARDS = true;
                take_cards(socket);
            }
            // nope if already took cards
            else {
                cards::TOOK_CARDS = false;
                play_nope(socket);
            }
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
            let action_names: Vec<String> = ["reset".to_string(), "invisible".to_string(), "nominate".to_string()].to_vec();
            unsafe{
                cards::TOOK_CARDS = false;
            }

            for card in &possible_cards {
                if action_names.contains(&card.type_field) {
                    let mut action_card_vec: Vec<Card> = [].to_vec();
                    action_card_vec.push(card.clone());
                    play_action(action_card_vec, socket, &opponent);
                    return true;
                }
            }

            discard_cards(possible_cards[..(decider.value.unwrap()) as usize].to_vec(), socket, opponent);
            return true;
        }
    }
    return false;
}

/// internal function to send play cards event
fn discard_cards(cards: Vec<Card>, socket: &RawClient, _opponent: &GamePlayer) {
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

fn play_action(cards: Vec<Card>, socket: &RawClient, opponent: &GamePlayer) {
    match cards.get(0).unwrap().type_field.trim_end() {
        "reset" => {
            discard_cards(cards, socket, opponent);
        }
        "invisible" => {
            discard_cards(cards, socket, opponent);
        }
        "nominate" => {
            let color = &cards.get(0).as_ref().unwrap().colors.as_ref().unwrap().get(0).unwrap();
            play_nominate(cards.clone(), socket, opponent, 1, color)
        }
        _ => println!("something went wrong in play_action")
    }
}

fn play_nominate(cards: Vec<Card>, socket: &RawClient, opponent: &GamePlayer, nominate_amount: i32, nominated_color: &str) {
    println!("playing {:?}", cards);
    let action_body = NominateAction {
        type_field: "nominate".to_string(),
        explanation: "playing a nominate card!".to_string(),
        amount: None,
        cards: Option::from(cards),
        player: None,
        nominatedPlayer: Option::from(opponent.clone()),
        nominatedAmount: Option::from(nominate_amount),
        nominatedColor: Option::from(nominated_color.to_string()),
    };

    let payload = json!(&action_body);
    println!("sending nominate card: {}", &payload);

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