use binance::{config::Config, websockets::*};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Sender;
use std::thread;


use crate::agents::binance_market_making::models::BidAskPair;

pub fn start_market_data_provider(config: &Config, sender: Sender<BidAskPair>) -> thread::JoinHandle<()> {
    let local_config = config.clone();
    thread::spawn(move || {
        let mut last_ask = 0.0;
        let mut last_bid = 0.0;
        let keep_running = AtomicBool::new(true); // Used to control the event loop
        let agg_trade = format!("btcusdt@bookTicker"); // All Symbols
        let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
            match event {
                // 24hr rolling window ticker statistics for all symbols that changed in an array.
                WebsocketEvent::BookTicker(event) => {
                    let best_ask = event.best_ask.parse::<f64>().unwrap();
                    let best_bid = event.best_bid.parse::<f64>().unwrap();

                    if last_ask == best_ask && last_bid == best_bid {
                        ()
                    }
                    last_ask = best_ask;
                    last_bid = best_bid;

                    let nbbo = BidAskPair::new(best_ask, best_bid);

                    match sender.send(nbbo) {
                        Err(error) => println!("ERROR: {}", error),
                        _ => ()
                    };
                }
                _ => println!("irrelevant event"),
            };

            Ok(())
        });

        web_socket.connect_with_config(&agg_trade, &local_config).unwrap(); // check error
        if let Err(e) = web_socket.event_loop(&keep_running) {
            match e {
                err => {
                    println!("Error: {:?}", err);
                }
            }
        }
    })
}
