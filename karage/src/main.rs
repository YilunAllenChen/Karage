use std::collections::HashMap;
use std::fmt;

use alpacars::models::SubscriptionMessage;
use alpacars::WSClient;
use dotenv_codegen::dotenv;

struct ArbAction {
    buy_eth: f64,
    buy_btc: f64,
    buy_ethbtc: f64,
    sell_ethbtc: f64,
}

#[derive(Debug)]
struct Quote {
    px: f64,
    qty: f64,
}

fn check_arb(prices: &HashMap<String, Quote>) -> Option<ArbAction> {
    let fund = 10000.0;
    let eth = match prices.get("ETH/USD") {
        Some(p) => p,
        None => return None,
    };
    let btc = match prices.get("BTC/USD") {
        Some(p) => p,
        None => return None,
    };
    let ethbtc = match prices.get("ETH/BTC") {
        Some(p) => p,
        None => return None,
    };
    let div = eth.px / btc.px;
    let _spread = (div - ethbtc.px).abs();
    println!("{:?}", _spread);
    let buy_eth = fund / eth.px;
    let buy_btc = fund / btc.px;
    let buy_ethbtc = buy_eth / ethbtc.px;
    let sell_ethbtc = buy_eth / ethbtc.px;

    Some(ArbAction {
        buy_btc,
        buy_eth,
        buy_ethbtc,
        sell_ethbtc,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut client = Client::new(dotenv!("ALPACA_API_KEY"), dotenv!("ALPACA_API_SECRET"));
    
    let coin1 = "SOL".to_string();
    let coin2 = "MATIC".to_string();
    let p1 = coin1.clone() + "/USD";
    let p2 = coin2.clone() + "/USD";
    let cc = coin1 + "/" + coin2.as_str();

    let mut ws_client = WSClient::new(
        dotenv!("ALPACA_API_KEY"),
        dotenv!("ALPACA_API_SECRET"),
        "wss://stream.data.alpaca.markets/v1beta2/crypto",
    );

    ws_client.subscribe(vec![p1, p2, cc]).unwrap();
    let mut latest_prices: HashMap<String, Quote> = HashMap::new();

    loop {
        let message = ws_client.get().unwrap();

        let res = serde_json::from_str::<Vec<SubscriptionMessage>>(&message.to_string());
        match res {
            Ok(msgs) => msgs.into_iter().for_each(|msg| {
                match msg {
                    SubscriptionMessage::SubscriptionQuote(quote) => {
                        let quote_to_record = match quote.symbol.as_str() {
                            // our buy, use ask
                            "ETH/USD" | "ETH/BTC" => Quote {
                                px: quote.bid_price,
                                qty: quote.bid_size,
                            },
                            "BTC/USD" => Quote {
                                px: quote.ask_price,
                                qty: quote.ask_size,
                            },
                            _ => {
                                println!("Unsupported pair");
                                Quote {
                                    px: 0.0,
                                    qty: 0.0
                                }
                            }
                        };
                        latest_prices.insert(quote.symbol, quote_to_record);
                        // println!("{:?}", latest_prices);
                        check_arb(&latest_prices);
                    }
                    SubscriptionMessage::Subscription(_)
                    | SubscriptionMessage::SubscriptionSuccess(_) => {}
                }
            }),
            Err(err) => {
                println!("{:?}\n{:?}", err, message.to_string());
            }
        }
    }
}
