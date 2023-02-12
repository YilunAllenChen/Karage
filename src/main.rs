mod agents;

use agents::binance_market_making;
fn main() {
    let join_handles = binance_market_making();
    for h in join_handles {
        h.join().unwrap();
    }
}
