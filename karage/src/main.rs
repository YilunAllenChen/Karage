use dotenv_codegen::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Account {
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
struct Bars {
    bars: Vec<Bar>,
    symbol: String,
    next_page_token: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct Bar {
    #[serde(rename="t")]
    timestamp: String,
    #[serde(rename="o")]
    open: f32,
    #[serde(rename="h")]
    high: f32,
    #[serde(rename="l")]
    low: f32,
    #[serde(rename="c")]
    close: f32,
    #[serde(rename="v")]
    volume: f64,
    #[serde(rename="n")]
    num_trades: f32,
    #[serde(rename="vw")]
    volume_weighted_price: f32
}


async fn alpaca_get(url: String) -> Result<String, reqwest::Error> {
    let alpaca_key = dotenv!("ALPACA_API_KEY");
    let alpaca_secret = dotenv!("ALPACA_API_SECRET");
    reqwest::Client::new()
        .get(url)
        .header("APCA-API-KEY-ID", alpaca_key)
        .header("APCA-API-SECRET-KEY", alpaca_secret)
        .send()
        .await?
        .text()
        .await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = dotenv!("ALPACA_BASE_URL");
    // let resp_text = alpaca_get(format!("{base_url}/v2/account"))
    //         .await?;
    // let res: Account = serde_json::from_str(&resp_text).unwrap();
    // println!("{res:?}");

    let market_data_base_url = "https://data.alpaca.markets";

    let resp_text = alpaca_get(format!("{market_data_base_url}/v2/stocks/aapl/bars?timeframe=60Min"))
        .await?;
    println!("{resp_text}");
    let bars: Bars = serde_json::from_str(&resp_text).unwrap();
    let firstbar = bars.bars.get(0);
    Ok(())
}
