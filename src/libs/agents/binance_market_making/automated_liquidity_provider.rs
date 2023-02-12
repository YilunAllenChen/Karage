use binance::api::*;
use binance::config::Config;
use binance::futures::account::{FuturesAccount};
use std::sync::mpsc::Receiver;
use std::thread;

use crate::libs::models::BidAskPair;
enum ACTION {
    RequoteBid,
    RequoteAsk,
    RequoteBoth,
    NOOP,
}

static EDGE: f64 = 10.0;
static BASE_QTY: f64 = 0.01;

pub fn start_automated_liquidity_provider(
    key: &String,
    secret: &String,
    config: &Config,
    receiver: Receiver<BidAskPair>,
) -> thread::JoinHandle<()> {
    let api_key = Some(key.to_string());
    let secret_key = Some(secret.to_string());
    let account: FuturesAccount = Binance::new_with_config(api_key, secret_key, config);

    thread::spawn(move || {
        let mut last_quote = BidAskPair::new(0.0, 0.0);
        for new_market_quote in receiver {
            let action = match (
                new_market_quote.best_ask == last_quote.best_ask,
                new_market_quote.best_bid == last_quote.best_bid,
            ) {
                (false, false) => ACTION::RequoteBoth,
                (true, false) => ACTION::RequoteBid,
                (false, true) => ACTION::RequoteAsk,
                (true, true) => ACTION::NOOP,
            };
            match action {
                ACTION::RequoteAsk => {
                    pull(&account);
                    trade(&account, -BASE_QTY, new_market_quote.best_ask + EDGE);
                }
                ACTION::RequoteBid => {
                    pull(&account);
                    trade(&account, BASE_QTY, new_market_quote.best_bid - EDGE);
                }
                ACTION::RequoteBoth => {
                    pull(&account);
                    trade(&account, -BASE_QTY, new_market_quote.best_ask + EDGE);
                    trade(&account, BASE_QTY, new_market_quote.best_bid - EDGE);
                }
                ACTION::NOOP => (),
            }
            last_quote = new_market_quote;
        }
    })
}

fn pull(account: &FuturesAccount) {
    match account.cancel_all_open_orders("BTCUSDT") {
        Ok(_) => println!("Pulled"),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn trade(account: &FuturesAccount, qty: f64, price: f64) {

    if qty < 0.0 {
        match account.limit_sell(
            "BTCUSDT".to_string(),
            qty.abs(),
            price.round(),
            binance::futures::account::TimeInForce::GTX,
        ) {
            Ok(_) => println!("Requote ask"),
            Err(e) => println!("Error: {:?}", e),
        }
    } else if qty > 0.0 {
        match account.limit_buy(
            "BTCUSDT".to_string(),
            qty,
            price.round(),
            binance::futures::account::TimeInForce::GTX,
        ) {
            Ok(_) => println!("Requote bid"),
            Err(e) => println!("Error: {:?}", e),
        }
    };
}
