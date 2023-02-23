use reqwest;
use serde_json::{from_str, Number, Result, Value};

enum Endpoint {
    StockId,
    BasicInfo,
}

impl Endpoint {
    fn as_str(&self) -> &'static str {
        match self {
            Self::StockId => "https://quotes-gw.webullbroker.com/api/search/pc/tickers?keyword={}&pageIndex=1&pageSize=20&regionId=06",
            Self::BasicInfo => "https://quotes-gw.webullfintech.com/api/stock/tickerRealTime/getQuote?tickerId={}&includeSecu=1&includeQuote=1&more=1"
        }
    }
}

pub fn get_basic_info(stock_symbol: &str) -> Result<Value> {
    let formatted = format!("https://quotes-gw.webullbroker.com/api/search/pc/tickers?keyword={}&pageIndex=1&pageSize=20&regionId=06", stock_symbol);

    let resp = reqwest::blocking::get(formatted).unwrap().text().unwrap();
    // println!("{resp}");
    let v: Value = from_str(&resp).unwrap();
    let ticker_id = v.get("data").unwrap().as_array().unwrap().into_iter().nth(0).unwrap().get("tickerId").unwrap();
    let formatted = format!("https://quotes-gw.webullfintech.com/api/stock/tickerRealTime/getQuote?tickerId={}&includeSecu=1&includeQuote=1&more=1", &ticker_id);
    let resp = reqwest::blocking::get(formatted).unwrap().text().unwrap();
    from_str(&resp)
}

pub fn get_pretty_info(stock_symbol: &str) -> Result<String> {
    let res = get_basic_info(stock_symbol).unwrap();
    serde_json::to_string_pretty(&res)
}