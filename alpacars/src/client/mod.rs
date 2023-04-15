use reqwest::Error;
use serde_json::{json, Value};
use std::fmt;
use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;

#[derive(Debug)]
pub struct ClientError {
    message: String,
}

impl fmt::Display for ClientError {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for ClientError {}

pub struct Client {
    key: String,
    secret: String,
}

pub struct WSClient {
    client: Client,
    websocket: WebSocket<MaybeTlsStream<TcpStream>>,
}

enum EndPoint {}

impl WSClient {
    pub fn new(key: &str, secret: &str, wss_endpoint: &str) -> Self {
        let (mut _websocket, _) =
            connect(Url::parse(wss_endpoint).unwrap()).expect("Can't connect");
        let auth_str = format!(
            r#"{{
                "action": "auth",
                "key": "{}",
                "secret": "{}"
            }}"#,
            key, secret
        );
        _websocket
            .write_message(Message::Text(auth_str.into()))
            .unwrap();
        WSClient {
            client: Client {
                key: String::from(key),
                secret: String::from(secret),
            },
            websocket: _websocket,
        }
    }

    pub fn subscribe(&mut self, pairs: Vec<String>) -> Result<String, ClientError> {
        let r = format!("{:?}", pairs);
        println!("{}", r);
        self.websocket
            .write_message(Message::Text(
                r#"{
                "action": "subscribe",
                "quotes": ["BTC/USD", "ETH/USD", "ETH/BTC"]
            }"#
                .into(),
            ))
            .unwrap();
        Ok(String::from("Success"))
    }

    pub fn get(&mut self) -> Result<Message, ClientError> {
        match self.websocket.read_message() {
            Ok(message) => Ok(message),
            Err(error) => Err(ClientError {
                message: error.to_string(),
            }),
        }
    }
}

impl Client {
    pub fn new(key: &str, secret: &str) -> Self {
        Client {
            key: String::from(key),
            secret: String::from(secret),
        }
    }

    pub fn ws_connect(&mut self, wss_endpoint: &str) {}

    async fn get(&self, url: String) -> Result<String, Error> {
        let resp = reqwest::Client::new()
            .get(url)
            .header("APCA-API-KEY-ID", &self.key)
            .header("APCA-API-SECRET-KEY", &self.secret)
            .send()
            .await?;
        resp.text().await
    }

    async fn post(&self, url: String, body: Value) -> Result<String, Error> {
        let resp = reqwest::Client::new()
            .post(url)
            .header("APCA-API-KEY-ID", &self.key)
            .header("APCA-API-SECRET-KEY", &self.secret)
            .body(serde_json::to_string(&body).unwrap())
            .send()
            .await?;
        resp.text().await
    }

    pub async fn get_crypto_quote(&self, currency_pair: &str) -> Result<String, Error> {
        self.get(format!(
            "https://data.alpaca.markets/v1beta2/crypto/latest/orderbooks?symbols={}",
            currency_pair
        ))
        .await
    }

    pub async fn crypto_market_order(
        &self,
        symbol: &str,
        qty: f64,
        side: &str,
        time_in_force: Option<&str>,
    ) -> Result<String, Error> {
        self.post(
            format!("https://paper-api.alpaca.markets/v2/orders",),
            json!({
                "symbol": symbol,
                "qty": qty.to_string(),
                "side": side,
                "type": "market",
                "time_in_force": time_in_force.unwrap_or("gtc")
            }),
        )
        .await
    }
}
