use std::sync::mpsc;

use binance::config::Config;
use dotenv_codegen::dotenv;

mod models;
mod modules;

use crate::modules::start_automated_liquidity_provider;
use crate::modules::start_market_data_provider;
use crate::models::BidAskPair;

fn main() {
    let key = dotenv!("BINANCE_API_KEY").to_owned();
    let secret = dotenv!("BINANCE_API_SECRET").to_owned();
    let config = Config::default()
        .set_rest_api_endpoint("https://testnet.binance.vision")
        .set_ws_endpoint("wss://testnet.binance.vision/ws")
        .set_futures_rest_api_endpoint("https://testnet.binancefuture.com")
        .set_futures_ws_endpoint("https://dstream.binancefuture.com");

    let (price_provider, price_consumer) = mpsc::channel::<BidAskPair>();

    let mdp_thread_handle = start_market_data_provider(&config, price_provider);
    let alp_thread_handle = start_automated_liquidity_provider(&key, &secret, &config, price_consumer);

    mdp_thread_handle.join().unwrap();
    alp_thread_handle.join().unwrap();    
}
