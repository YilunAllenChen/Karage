use crate::models::BidAskPair;
use binance::api::*;
use binance::config::Config;
use binance::futures::account::FuturesAccount;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;


pub fn start_position_monitor(
    key: &String,
    secret: &String,
    config: &Config,
    receiver: Receiver<BidAskPair>,
) -> thread::JoinHandle<()> {
    let api_key = Some(key.to_string());
    let secret_key = Some(secret.to_string());
    let account: FuturesAccount = Binance::new_with_config(api_key, secret_key, config);

    thread::spawn(move || {
        loop {
            let _ = account.position_information("BTCUSDT");
            thread::sleep(Duration::from_secs(1));
        }
    })
}