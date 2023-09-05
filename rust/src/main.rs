use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::{broadcast, mpsc};
use tokio::task;

use calculations::{rsi, vwap};
use subscriptions::{subscribe_binance_candles, subscribe_bitfinex_candles};

mod calculations;
mod candlesticks;
mod subscriptions;

#[tokio::main]
async fn main() {
    let (binance_sender, mut binance_receiver) = mpsc::channel(1024);
    let (bitfinex_sender, mut bitfinex_receiver) = mpsc::channel(1024);

    let (exit_sender, exit_receiver1) = broadcast::channel(1);
    let exit_receiver2 = exit_sender.subscribe();

    // Spawn a task for subscribing to exchanges
    let binance_handle = task::spawn(subscribe_binance_candles(binance_sender, exit_receiver1));
    let bitfinex_handle = task::spawn(subscribe_bitfinex_candles(bitfinex_sender, exit_receiver2));

    let mut rsi_calculator = rsi::RsiCalculator::new(14);
    let mut vwap_calculator = vwap::VwapCalculator::new();

    // Wait for a shutdown signal
    let mut signal = signal(SignalKind::interrupt()).expect("Failed to create signal");

    loop {
        tokio::select! {
            _ = signal.recv() => {
                // Send the exit signal to the tasks
                if let Err(err) = exit_sender.send(()) {
                    eprintln!("Failed to send exit signal: {:?}", err);
                }
                println!("Gracefully Exiting...");
                // Wait for the tasks to finish
                if let Err(err) = binance_handle.await {
                    eprintln!("Binance exit error: {:?}", err);
                }
                if let Err(err) = bitfinex_handle.await {
                    eprintln!("Bitfinex exit error: {:?}", err);
                }
                break; // Exit the main loop
            }
            Some(candlestick) = binance_receiver.recv() => {
                // Handle received binance candlestick data
                // println!(
                //     "Binance. Time: {}, Open: {:.2}, High: {:.2}, Low: {:.2}, Close: {:.2}, Volume: {:.2}",
                //     candlestick.st, candlestick.o, candlestick.h, candlestick.l, candlestick.c, candlestick.v
                // );
                if candlestick.x {  // if it is closing record
                    if let Some(rsi) = rsi_calculator.update(candlestick.c) {
                        println!("Binance\tClose Price: {:.4}\tRSI: {:.4}", candlestick.c, rsi);
                    }
                }
            }

            Some(candlestick) = bitfinex_receiver.recv() => {
                // Handle received bitfinex candlestick data
                // println!(
                //     "Bitfinex. Time: {}, Open: {:.2}, High: {:.2}, Low: {:.2}, Close: {:.2}, Volume: {:.2}",
                //     candlestick.mts, candlestick.open, candlestick.high, candlestick.low, candlestick.close, candlestick.volume
                // );
                let vwap = vwap_calculator.update((candlestick.open+candlestick.close)/2.0, candlestick.volume);
                println!("Bitfinex\tClose Price: {:.4}\tVWAP: {:.4}", candlestick.close, vwap);
            }
        }
    }
}
