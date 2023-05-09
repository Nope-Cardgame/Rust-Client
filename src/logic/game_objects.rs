use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub state: String,
    pub noActionCards: bool,
    pub noWildCards: bool,
    pub oneMoreStartCard: bool,
    pub players: Vec<GamePlayer>,
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