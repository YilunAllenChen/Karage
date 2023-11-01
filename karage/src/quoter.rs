use apca::{api::v2::order, data::v2::stream::Quote, Client};
use num_decimal::Num;
use tokio::sync::mpsc::Receiver;

async fn quote(client: &Client, ticker: &str, around: &Num, spread: &Num, quantity: &Num) {
    let ask_price = Num::from(around + spread);
    let bid_price = Num::from(around - spread);
    let stop_loss = Num::from(around - spread * 2);

    let quote = order::OrderReqInit {
        type_: order::Type::Limit,
        class: order::Class::Bracket,
        limit_price: Some(bid_price),
        stop_loss: Some(order::StopLoss::Stop(stop_loss)),
        take_profit: Some(order::TakeProfit::Limit(ask_price)),
        ..Default::default()
    }
    .init(
        ticker,
        order::Side::Buy,
        order::Amount::quantity(quantity.clone()),
    );

    let quote_fut = client.issue::<order::Post>(&quote).await;
    println!(
        "Quoting {}x {} @ {} spr {}, error {:?}",
        quantity,
        ticker,
        around,
        spread,
        quote_fut.err()
    );
}

pub async fn quoter(client: Client, ticker: String, mut rx: Receiver<Quote>) {
    let spread = Num::new(1, 1);
    let sensitivity = Num::new(3, 2);
    let qty = Num::new(1, 1);
    let mut last_theo: Option<Num> = None;

    while let Some(message) = rx.recv().await {
        let new_theo = Num::from((&message.ask_price + &message.bid_price) / 2);
        let around = new_theo.clone().round_with(2);
        match last_theo.as_ref() {
            None => {
                quote(&client, &ticker, &around, &spread, &qty).await;
                last_theo = Some(new_theo);
            }
            Some(old_theo) => {
                let theo_diff = f64::abs((&new_theo - old_theo).to_f64().unwrap());
                if theo_diff > sensitivity.to_f64().unwrap() {
                    println!(
                        "Theo drift: {} ({} -> {}): Requoting",
                        theo_diff, new_theo, old_theo
                    );
                    quote(&client, &ticker, &around, &spread, &qty).await;
                    last_theo = Some(new_theo);
                }
            }
        }
    }
}
