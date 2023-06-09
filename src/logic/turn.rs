use rust_socketio::RawClient;
use serde_json::json;
use crate::logic::game_objects::{Card, DiscardAction, Game, GamePlayer, NominateAction, NominateActionMulti, NopeAction, TakeAction};

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
    let mut opponent = players.get(1).unwrap().clone();

    for player in  players {
        if player.username == dotenvy::var("AUTH_USER").expect("error retrieving username from .env - create_game()") {
            cards::CARDS = player.cards.unwrap();
        }
        else if player.clone().cardAmount.unwrap() > 0 {
            opponent = player;
        }

    }

    // get the first card of the discard pile and regard is as the deciding card
    let discard_pile = game_state.clone().discardPile.unwrap();
    let mut decider = discard_pile.get(0).unwrap().clone();

    // match the decider type
    match decider.type_field.trim_end() {
        // search if hand has the right cards
        "number" => {
            discard_loop(&decider, &opponent, socket);
        }

        // action cards section

        "reset" => {
            println!("Received reset card!");

            // just take first available card
            let mut play_first: Vec<Card> = [].to_vec();
            play_first.push(cards::CARDS[0].clone());
            if play_first.get(0).unwrap().type_field == "number" {
                discard_cards(play_first, socket, &opponent);
            }
            else {
                play_action(play_first, socket, &opponent);
            }

        }
        "invisible" => {
            println!("Received invis card!");
            // if invis card is not only card on discard pile, check what is underneath
            if discard_pile.len() > 1 {
                let decider_index = check_invis_amount(discard_pile.clone());
                decider = discard_pile.get(decider_index).unwrap().clone();

                if decider.clone().type_field == "number"{
                    discard_loop(&decider, &opponent, socket);
                }
                else {
                    action_under_invis(decider, discard_pile.clone(),game_state , &opponent, socket);
                }

            }
            // otherwise invis is decider color with value 1
            else {
                let topcard = discard_pile.get(0).unwrap();
                decider = Card {
                    type_field: topcard.type_field.clone(),
                    value: Option::from(1),
                    colors: topcard.colors.clone(),
                    name: topcard.name.clone(),
                };
                discard_loop(&decider, &opponent, socket);
            }

        }
        "nominate" => {
            println!("Received nominate card!");

            // if nominate is first card in game
            if discard_pile.len() == 1 && game_state.clone().state.clone().unwrap() == "nominate_flipped" {
                let nominate_card = discard_pile.get(0).unwrap().clone();
                let mut card_vec: Vec<Card> = [].to_vec();
                card_vec.push(nominate_card.clone());

                if nominate_card.clone().colors.unwrap().len() > 1 {
                    play_nominate_multi(None, socket, &opponent, 1, nominate_card.colors.unwrap().get(0).unwrap().as_ref())
                }
                else {
                    play_nominate(None, socket, &opponent, 1)
                }
            }
            // if opponent played nominate
            else {
                // get nominated color
                let mut color_vec: Vec<String> = [].to_vec();

                if discard_pile.get(0).unwrap().clone().colors.unwrap().len() > 1 {
                    color_vec.push(game_state.lastNominateColor.clone().unwrap());
                }
                else {
                    color_vec.push(discard_pile.get(0).unwrap().clone().colors.unwrap().get(0).unwrap().clone())
                }


                let nominate_decider: Card = Card {
                    type_field: "number".to_string(),
                    value: game_state.lastNominateAmount.clone(),
                    colors: Option::from(color_vec),
                    name: decider.name.clone(),
                };
                discard_loop(&nominate_decider, &opponent, socket);
            }
        }
        _ => {
            println!("invalid card type received from server!");
        }

    }
}

/// checks for the first non inis card under one or multiple invis cards
fn check_invis_amount(discard_pile: Vec<Card>) -> usize {
    let mut index = 0;
    'check_loop: for (card_index, card) in discard_pile.iter().enumerate() {
        if card.type_field.trim_end() == "invisible" {
            index = card_index;
        }
        else {
            break 'check_loop;
        }
    }

    return index + 1;
}

/// special action decider if action card was under invis card(s)
fn action_under_invis(decider: Card, discard_pile: Vec<Card>, game_state: &Game, opponent: &GamePlayer, socket: &RawClient) {
    match decider.clone().type_field.trim_end() {

        "reset" => unsafe {
            println!("Received reset card!");

            // just take first available card
            let mut play_first: Vec<Card> = [].to_vec();
            play_first.push(cards::CARDS[0].clone());
            if play_first.get(0).unwrap().type_field == "number" {
                discard_cards(play_first, socket, &opponent);
            }
            else {
                play_action(play_first, socket, &opponent);
            }

        }

        // nominate block doesn't need special consideration as normal loop, as it can only be at least the second card in discard pile
        "nominate" => {
            println!("Received nominate card!");

            let mut color_vec: Vec<String> = [].to_vec();

            if discard_pile.get(0).unwrap().clone().colors.unwrap().len() > 1 {
                color_vec.push(game_state.lastNominateColor.clone().unwrap());
            }
            else {
                color_vec.push(discard_pile.get(0).unwrap().clone().colors.unwrap().get(0).unwrap().clone())
            }


            let nominate_decider: Card = Card {
                type_field: "number".to_string(),
                value: game_state.lastNominateAmount.clone(),
                colors: Option::from(color_vec),
                name: decider.name.clone(),
            };
            discard_loop(&nominate_decider, &opponent, socket);

        }
        _ => {
            println!("invalid card type received from server!");
        }

    }
}

/// controls if card could be played
/// otherwise draw cards or play nope if already drew cards
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
    println!("Playing number cards!");
    println!("Current decider: {:?}", &decider);
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



        if possible_cards.len() >= (decider.value.expect("Something went wrong unwrapping decider value")) as usize {
            let action_names: Vec<String> = ["reset".to_string(), "invisible".to_string(), "nominate".to_string()].to_vec();
            unsafe{
                cards::TOOK_CARDS = false;
            }

            possible_cards.sort_by(|a, b| b.colors.as_ref().unwrap().len().cmp(&a.colors.as_ref().unwrap().len()));

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
fn discard_cards(mut cards: Vec<Card>, socket: &RawClient, _opponent: &GamePlayer) {
    // if opponent has many cards, sort in ascending order, to keep high value cards
    // if opponent.cardAmount.unwrap() > 4 {
    //     cards.sort_by(|a, b| a.value.unwrap().cmp(&b.value.unwrap()));
    // }
    // // if opponent has 4 or fewer cards, sort in descending order to keep low value cards
    // else {
        cards.sort_by(|a, b| b.value.unwrap().cmp(&a.value.unwrap()));
    // }

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

/// controls which action should be played
fn play_action(cards: Vec<Card>, socket: &RawClient, opponent: &GamePlayer) {
    match cards.get(0).unwrap().type_field.trim_end() {
        "reset" => {
            discard_cards(cards, socket, opponent);
        }
        "invisible" => {
            discard_cards(cards, socket, opponent);
        }
        "nominate" => {
            let mut nom_amount = 1;
            println!("{:?}", opponent);

            if !(opponent.disqualified.as_ref().unwrap()) && !&(opponent.cardAmount.as_ref().unwrap() == &0) {
                if opponent.cardAmount.as_ref().unwrap() > &6 {
                    nom_amount = 3
                }
                else if opponent.cardAmount.as_ref().unwrap() > &3 {
                    nom_amount = 2
                }
            }


            if cards.clone().get(0).clone().unwrap().colors.clone().unwrap().len() > 1 {
                play_nominate_multi(Option::from(cards.clone()), socket, &opponent, nom_amount,
                                    cards.clone().get(0).clone().unwrap().colors.clone().unwrap().get(0).unwrap().as_ref())
            }
            else {
                play_nominate(Option::from(cards.clone()), socket, &opponent, nom_amount)
            }
            // play_nominate_multi(cards.clone(), socket, opponent, 1, color)
        }
        _ => println!("something went wrong in play_action")
    }
}

/// send nominate event from multi-color nominate card to server
fn play_nominate_multi(cards_to_send: Option<Vec<Card>>, socket: &RawClient, opponent: &GamePlayer, nominate_amount: i32, nominated_color: &str) {
    println!("playing {:?}", cards_to_send);
    let action_body = NominateActionMulti {
        type_field: "nominate".to_string(),
        explanation: "playing a multi-color nominate card!".to_string(),
        amount: None,
        cards: cards_to_send,
        player: None,
        nominatedPlayer: Option::from(opponent.clone()),
        nominatedAmount: Option::from(nominate_amount),
        nominatedColor: Option::from(nominated_color.to_string()),
    };

    let payload = json!(&action_body);
    println!("sending multi-color nominate card: {}", &payload);

    socket.emit("playAction", payload).expect("error in sending played cards");
}

/// send nominate event to server
fn play_nominate(cards_to_send: Option<Vec<Card>>, socket: &RawClient, opponent: &GamePlayer, nominate_amount: i32) {

    println!("playing {:?}", cards_to_send);
    let action_body = NominateAction {
        type_field: "nominate".to_string(),
        explanation: "playing a single color nominate card!".to_string(),
        amount: None,
        cards: cards_to_send,
        player: None,
        nominatedPlayer: Option::from(opponent.clone()),
        nominatedAmount: Option::from(nominate_amount),
    };

    let payload = json!(&action_body);
    println!("sending single-color nominate card: {}", &payload);

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