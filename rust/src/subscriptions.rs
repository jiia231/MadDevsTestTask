use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use tokio::sync::{broadcast, mpsc};
use tokio::time::Duration;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::candlesticks::*;

pub async fn subscribe_binance_candles(
    sender: mpsc::Sender<BinanceCandlestick>,
    mut exit_receiver: broadcast::Receiver<()>, // Changed the type of exit_receiver
) {
    let url = "wss://stream.binance.com:9443/ws/btcusdt@kline_5m";
    let (mut ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to Binance");
    let mut interval = tokio::time::interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = exit_receiver.recv() => {
                println!("Closing Binance WebSocket");
                let _ = ws_stream.close(None).await;
                break;
            }
            _ = interval.tick() => {
                if let Some(msg) = ws_stream.next().await {
                    if let Ok(msg) = msg {
                        if let Message::Text(data) = msg {
                            let candlestick = serde_json::from_str::<BinanceCandlestickData>(&data).expect("Couldn't parse data");
                            sender.send(candlestick.k).await.expect("Failed to send to channel");
                        }
                    }
                }
            }
        }
    }
}

pub async fn subscribe_bitfinex_candles(
    sender: mpsc::Sender<BitfinexCandlestick>,
    mut exit_receiver: broadcast::Receiver<()>,
) {
    let url = "wss://api-pub.bitfinex.com/ws/2";
    let (mut ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to Bitfinex");
    let _info_msg = ws_stream.next().await;
    let _ = ws_stream.send(Message::from("{\"event\":\"subscribe\",\"channel\":\"candles\",\"key\":\"trade:1m:tBTCUSD\"}")).await;
    let _subscription_msg = ws_stream.next().await;
    if let Some(Ok(snapshot_msg)) = ws_stream.next().await {
        if let Message::Text(data) = snapshot_msg {
            let snapshot = serde_json::from_str::<BitfinexCandlestickSnapshotData>(&data).expect("Couldn't parse data");
            for candlestick in snapshot.candle_data.iter() {
                sender.send(candlestick.clone()).await.expect("Failed to send to channel");
            }
        }
    };


    let mut interval = tokio::time::interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = exit_receiver.recv() => {
                println!("Closing Bitfinex WebSocket");
                let _ = ws_stream.close(None).await;
                break;
            }
            _ = interval.tick() => {
                if let Some(msg) = ws_stream.next().await {
                    if let Ok(msg) = msg {
                        if let Message::Text(data) = msg {
                            if data.contains("\"event\"") || data.contains("hb") { continue }
                            let candlestick = serde_json::from_str::<BitfinexCandlestickUpdateData>(&data).expect("Couldn't parse data");
                            sender.send(candlestick.candle_data.clone()).await.expect("Failed to send to channel");
                        }
                    }
                }
            }
        }
    }
}