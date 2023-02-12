use std::sync::mpsc;
use std::thread::JoinHandle;

mod automated_liquidity_provider;
mod market_data_provider;
// mod position_monitor;

use automated_liquidity_provider::start_automated_liquidity_provider;
use market_data_provider::start_market_data_provider;
// pub use position_monitor::start_position_monitor;

use binance::config::Config;
use dotenv_codegen::dotenv;

#[path ="../../models/mod.rs"]
mod models;
use models::BidAskPair;

pub fn binance_market_making() -> Vec<JoinHandle<()>>{
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

    vec![mdp_thread_handle, alp_thread_handle]
}