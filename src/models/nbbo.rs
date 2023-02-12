use std::fmt::Display;

#[derive(Debug)]
pub struct BidAskPair {
    pub best_ask: f64,
    pub best_bid: f64,
}

impl Display for BidAskPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {}", self.best_bid, self.best_ask)
    }
}

impl BidAskPair {
    pub fn new(best_ask: f64, best_bid: f64) -> Self {
        Self {
            best_ask: best_ask,
            best_bid: best_bid,
        }
    }
}
