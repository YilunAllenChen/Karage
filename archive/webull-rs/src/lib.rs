use reqwest::{self, blocking::Client};
use serde_json::{from_str, json, Number, Result, Value};
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

pub fn get_stock_id(stock_symbol: &str) -> i64 {
    let formatted = format!("https://quotes-gw.webullbroker.com/api/search/pc/tickers?keyword={}&pageIndex=1&pageSize=20&regionId=06", stock_symbol);

    let resp = reqwest::blocking::get(formatted).unwrap().text().unwrap();
    // println!("{resp}");
    let v: Value = from_str(&resp).unwrap();
    v.get("data")
        .unwrap()
        .as_array()
        .unwrap()
        .into_iter()
        .nth(0)
        .unwrap()
        .get("tickerId")
        .unwrap()
        .as_i64()
        .unwrap()
}

pub fn get_basic_info(stock_symbol: &str) -> Result<Value> {
    let ticker_id = get_stock_id(stock_symbol);
    let formatted = format!("https://quotes-gw.webullfintech.com/api/stock/tickerRealTime/getQuote?tickerId={}&includeSecu=1&includeQuote=1&more=1", &ticker_id);
    let resp = reqwest::blocking::get(formatted).unwrap().text().unwrap();
    from_str(&resp)
}

pub fn get_options(stock_symbol: &str) -> Result<Value> {
    let ticker_id = get_stock_id(stock_symbol);
    let formatted = format!("https://quotes-gw.webullfintech.com/api/quote/option/strategy/list");
    let body = json!({
        "tickerId":913254999,
        "count":-1,
        "direction":"all",
        "expireCycle":[3,2,4],
        "type":0,
        "quoteMultiplier":100
    })
    .to_string();
    let client = Client::new();
    let resp = client
        .post(formatted)
        .body(body)
        .header("Content-Type", "application/json")
        .header(
            "access_token",
            "dc_us_tech1.1866a4f9aa7-44531e0f2368464494506ceeab3059b0",
        )
        .send()
        .unwrap()
        .text().unwrap();
    println!("{resp:?}");
    from_str("")
}

pub fn trade_msft() -> Result<Value> {
    let url = "https://act.webullfintech.com/webull-paper-center/api/paper/1/acc/2492933/orderop/place/913323997";
    let client = Client::new();
    let body = json!({
      "shortSupport": true,
      "orderType": "LMT",
      "timeInForce": "DAY",
      "quantity": 1,
      "outsideRegularTradingHour": true,
      "action": "BUY",
      "tickerId": 913323997,
      "lmtPrice": "254.72",
      "comboType": "NORMAL",
      "serialId": "087accfc-39f0-4321-b2ab-dd0801f3a91a"
    })
    .to_string();
    let resp = client
        .post(url)
        .body(body)
        .header("Content-Type", "application/json")
        .header(
            "access_token",
            "dc_us_tech1.1866a4f9aa7-44531e0f2368464494506ceeab3059b0",
        )
        .send()
        .unwrap()
        .text()
        .unwrap();
    println!("{resp:?}");
    from_str(&resp)
}
