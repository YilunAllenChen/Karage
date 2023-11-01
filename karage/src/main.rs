use dotenv::dotenv;
use tokio::{join, spawn, sync::mpsc, task::spawn_local};

mod market_data;
mod quoter;

use apca::{ApiInfo, Client};

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    let api_info = ApiInfo::from_env().unwrap();
    let (sender, receiver) = mpsc::channel(32); // Create a channel with a buffer size of 32.

    let ticker = "TSLA".to_string();

    let market_data = spawn(market_data::stream_market_data(
        Client::new(api_info.clone()),
        vec![ticker.clone()],
        sender,
    ));
    let quoter = spawn(quoter::quoter(Client::new(api_info), ticker, receiver));
    let _ = join!(market_data, quoter);
}
