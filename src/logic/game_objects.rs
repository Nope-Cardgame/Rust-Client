use serde::{Deserialize, Serialize};

// this file contains all Game Object structs
// structs are defined in a way, that serde_json can autotransform from a json string

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub state: String,
    pub noActionCards: bool,
    pub noWildCards: bool,
    pub oneMoreStartCard: bool,
    pub players: Vec<GamePlayer>,
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GamePlayer {
    pub username: String,
    pub socketId: String,
    pub cardAmount: i64,
    pub disqualified: bool,
    pub accepted: bool,
    pub cards: Option<Vec<Card>>,
    pub ranking: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TournamentPlayer {
    pub username: String,
    pub ranking: i64,
    pub disqualified: bool,
    pub score: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tournament {
    pub id: String,
    pub mode: TournamentMode,
    pub participants: Vec<TournamentPlayer>,
    pub games: Vec<Game>,
    pub startTime: String,
    pub endTime: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TournamentMode {
    pub name: String,
    pub numberOfRounds: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: i64,
    pub color: Vec<String>,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WildCard {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: i64,
    pub color: Vec<String>,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardNominate {
    #[serde(rename = "type")]
    pub type_field: String,
    pub color: Vec<String>,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "type")]
    pub type_field: String,
    pub explanation: String,
    pub amount: Option<i64>,
    pub cards: Option<Card>,
    pub player: Option<GamePlayer>,
    pub nominatedPlayer: Option<GamePlayer>,
    pub nominatedColor: Option<String>,
}