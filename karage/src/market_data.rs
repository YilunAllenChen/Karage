use apca::{
    data::v2::stream::{drive, Bar, Data, MarketData, Quote, RealtimeData, Trade, IEX},
    Client, Error,
};
use futures::{stream::iter, FutureExt, StreamExt, TryStreamExt};
use tokio::sync::mpsc::Sender;

pub async fn stream_market_data(client: Client, tickers: Vec<String>, sender: Sender<Quote>) {
    let (mut stream, mut subscription) = client
        .subscribe::<RealtimeData<IEX, Bar, Quote, Trade>>()
        .await
        .unwrap();

    let mut data = MarketData::default();
    data.set_quotes(tickers);
    let subscribe = subscription.subscribe(&data).boxed();

    let () = drive(subscribe, &mut stream)
        .await
        .unwrap()
        .unwrap()
        .unwrap();

    stream
        // Stop after receiving and printing 50 updates.
        .for_each(|res| async {
            let maybe = res.unwrap().unwrap();
            match maybe {
                Data::Quote(quote) => {
                    sender.send(quote).await.unwrap();
                }
                _ => (),
            }
        })
        .await;

    ()
}
