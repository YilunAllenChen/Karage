mod libs;

// use libs::alpaca_pair_trading;
// use libs::binance_market_making;

use std::io::{stdout, Write};
use curl::easy::Easy;

fn main() {
    let mut easy = Easy::new();
    easy.url("https://data.alpaca.markets/v2/stocks/AAPL/trades").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}