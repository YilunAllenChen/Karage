use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Level {
    #[serde(rename = "p")]
    price: f64,
    #[serde(rename = "s")]
    quantity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    #[serde(rename = "a")]
    best_ask: Level,
    #[serde(rename = "b")]
    best_bid: Level,
    #[serde(rename = "t")]
    timestamp: String,
}