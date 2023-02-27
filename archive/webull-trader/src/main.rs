use webull_rs::{get_basic_info, get_options, trade_msft};
use serde_json::to_string_pretty;

fn main() {
    // let res = trade_msft().unwrap();
    let res = trade_msft().unwrap();
    match get_options("AAPL") {
        Ok(res) => println!("{}", to_string_pretty(&res).unwrap()),
        Err(err) => println!("{}", err)
    };
}
