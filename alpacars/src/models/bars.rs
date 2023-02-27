use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Bars {
    bars: Vec<Bar>,
    symbol: String,
    next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Bar {
    #[serde(rename = "t")]
    timestamp: String,
    #[serde(rename = "o")]
    open: f32,
    #[serde(rename = "h")]
    high: f32,
    #[serde(rename = "l")]
    low: f32,
    #[serde(rename = "c")]
    close: f32,
    #[serde(rename = "v")]
    volume: f64,
    #[serde(rename = "n")]
    num_trades: f32,
    #[serde(rename = "vw")]
    volume_weighted_price: f32,
}
