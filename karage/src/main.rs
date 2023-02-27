use alpacars::{Client, WSClient};
use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new(dotenv!("ALPACA_API_KEY"), dotenv!("ALPACA_API_SECRET"));

    let mut ws_client = WSClient::new(
        dotenv!("ALPACA_API_KEY"),
        dotenv!("ALPACA_API_SECRET"),
        "wss://stream.data.alpaca.markets/v1beta2/crypto",
    );

    ws_client.subscribe().unwrap();

    loop {
        let message = ws_client.get().unwrap();
        println!("{message}");
    }

    // let resp = client.get_crypto_quote("BTC/USD").await.unwrap();

    // println!("{resp}");
    // let resp = client.crypto_market_order("BTC/USD", 0.1, "buy", None).await.unwrap();
    // println!("{resp}");

    Ok(())
}
