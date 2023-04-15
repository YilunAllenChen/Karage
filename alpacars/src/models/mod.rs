use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    id: String,
    account_number: String,
    status: String,
    crypto_status: String,
    currency: String,
    buying_power: String,
    regt_buying_power: String,
    daytrading_buying_power: String,
    effective_buying_power: String,
    non_marginable_buying_power: String,
    bod_dtbp: String,
    cash: String,
    accrued_fees: String,
    pending_transfer_in: String,
    portfolio_value: String,
    pattern_day_trader: bool,
    trading_blocked: bool,
    transfers_blocked: bool,
    account_blocked: bool,
    created_at: String,
    trade_suspended_by_user: bool,
    multiplier: String,
    shorting_enabled: bool,
    equity: String,
    last_equity: String,
    long_market_value: String,
    short_market_value: String,
    position_market_value: String,
    initial_margin: String,
    maintenance_margin: String,
    last_maintenance_margin: String,
    sma: String,
    daytrade_count: i8,
    balance_asof: String,
    crypto_tier: i8,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionQuote {
    #[serde(rename = "T")]
    pub type_: String,
    #[serde(rename = "S")]
    pub symbol: String,
    #[serde(rename = "bp")]
    pub bid_price: f64,
    #[serde(rename = "ap")]
    pub ask_price: f64,
    #[serde(rename = "bs")]
    pub bid_size: f64,
    #[serde(rename = "as")]
    pub ask_size: f64,
    #[serde(rename = "t")]
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionSuccess {
    #[serde(rename = "T")]
    type_: String,
    #[serde(rename = "msg")]
    symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "T")]
    type_: String,
    trades: Vec<String>,
    quotes: Vec<String>,
    orderbooks: Vec<String>,
    bars: Vec<String>,
    #[serde(rename = "updatedBars")]
    updated_bars: Vec<String>,
    #[serde(rename = "dailyBars")]
    daily_bars: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubscriptionMessage {
    SubscriptionQuote(SubscriptionQuote),
    Subscription(Subscription),
    SubscriptionSuccess(SubscriptionSuccess),
}
