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

static EDGE: f64 = 50.0;
static MIN_EDGE: f64 = EDGE * 0.05;
static BASE_QTY: f64 = 0.1;

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
                (new_market_quote.best_ask-last_quote.best_ask).abs() < MIN_EDGE,
                (new_market_quote.best_bid-last_quote.best_bid).abs() < MIN_EDGE,
            ) {
                (false, false) => ACTION::RequoteBoth,
                (true, false) => ACTION::RequoteBid,
                (false, true) => ACTION::RequoteAsk,
                (true, true) => ACTION::NOOP,
            };
            match action {
                ACTION::RequoteAsk => {
                    let our_new_ask_quote = new_market_quote.best_ask + EDGE;
                    pull(&account);
                    trade(&account, -BASE_QTY, our_new_ask_quote);
                    last_quote.best_ask = our_new_ask_quote;
                }
                ACTION::RequoteBid => {
                    let our_new_bid_quote = new_market_quote.best_bid - EDGE;
                    pull(&account);
                    trade(&account, BASE_QTY, our_new_bid_quote);
                    last_quote.best_bid = our_new_bid_quote;
                }
                ACTION::RequoteBoth => {
                    let our_new_bid_quote = new_market_quote.best_bid - EDGE;
                    let our_new_ask_quote = new_market_quote.best_ask + EDGE;
                    pull(&account);
                    trade(&account, -BASE_QTY, our_new_ask_quote);
                    trade(&account, BASE_QTY, our_new_bid_quote);
                    last_quote.best_bid = our_new_bid_quote;
                    last_quote.best_ask = our_new_ask_quote;
                }
                ACTION::NOOP => (),
            }
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
