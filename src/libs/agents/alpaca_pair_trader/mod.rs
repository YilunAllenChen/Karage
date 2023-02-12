use alpaca_finance::{Account, Alpaca};
use dotenv_codegen::dotenv;

pub async fn alpaca_pair_trading() {
    let key = dotenv!("ALPACA_API_KEY").to_owned();
    let secret = dotenv!("ALPACA_API_SECRET").to_owned();

    let alpaca = Alpaca::live(&key, &secret).await.unwrap();
    let account = Account::get(&alpaca).await.unwrap();

    println!("I have ${:.2} in my account.", account.cash)
}
