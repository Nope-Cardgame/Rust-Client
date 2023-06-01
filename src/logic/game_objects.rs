use serde::{Deserialize, Serialize};

// this file contains all Game Object structs
// structs are defined in a way, that serde_json can autotransform from a json string

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Game {
    pub id: Option<String>,
    pub state: Option<String>,
    pub noActionCards: Option<bool>,
    pub noWildCards: Option<bool>,
    pub oneMoreStartCard: Option<bool>,
    pub players: Option<Vec<GamePlayer>>,
    pub tournament: Option<Tournament>,
    pub gameRole: Option<String>,
    pub encounterRound: Option<i64>,
    pub discardPile: Option<Vec<Card>>,
    pub lastAction: Option<Action>,
    pub currentPlayer: Option<GamePlayer>,
    pub startTime: Option<String>,
    pub initialTopCard: Option<Card>,
    pub actions: Option<Action>,
    pub endTime: Option<String>,
    pub actionTimeout: Option<i32>,
    pub invitationTimeout: Option<i32>,
    pub startWithRejection: Option<bool>,
    pub playerAmount: Option<i32>,
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct GamePlayer {
    pub username: String,
    pub socketId: Option<String>,
    pub cardAmount: Option<i64>,
    pub disqualified: Option<bool>,
    pub accepted: Option<bool>,
    pub cards: Option<Vec<Card>>,
    pub ranking: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TournamentPlayer {
    pub username: Option<String>,
    pub ranking: Option<i64>,
    pub disqualified: bool,
    pub score: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Tournament {
    pub id: Option<String>,
    pub mode: TournamentMode,
    pub participants: Option<Vec<TournamentPlayer>>,
    pub games: Option<Vec<Game>>,
    pub startTime: Option<String>,
    pub endTime: Option<String>,
    pub actionTimeout: Option<i32>,
    pub invitationTimeout: Option<i32>,
    pub startWithRejection: Option<bool>,
    pub sendGameInvite: Option<bool>,
    pub participantAmount: Option<i32>,
    pub gameAmount: Option<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct TournamentMode {
    pub name: Option<String>,
    pub numberOfRounds: Option<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Option<i64>,
    pub colors: Option<Vec<String>>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WildCard {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Option<i64>,
    pub color: Vec<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardNominate {
    #[serde(rename = "type")]
    pub type_field: String,
    pub color: Vec<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Action {
    #[serde(rename = "type")]
    pub type_field: String,
    pub explanation: Option<String>,
    pub cardAmount: Option<i64>,
    pub cards: Option<Vec<Card>>,
    pub player: Option<GamePlayer>,
    pub nominatedPlayer: Option<GamePlayer>,
    pub nominatedColor: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct DiscardAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub explanation: String,
    pub cards: Option<Vec<Card>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct NopeAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub explanation: String,
    pub player: Option<GamePlayer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct TakeAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub explanation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct NominateAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub explanation: String,
    pub amount: Option<i32>,
    pub cards: Option<Vec<Card>>,
    pub player: Option<GamePlayer>,
    pub nominatedPlayer: Option<GamePlayer>,
    pub nominatedAmount: Option<i32>,
    pub nominatedColor: Option<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Ready {
    #[serde(rename = "type")]
    pub type_field: String,
    pub accept: bool,
    pub inviteId: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Eliminated {
    pub disqualified: bool,
    pub reason: String,
}