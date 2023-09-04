# Test Task for MadDevs

## Description
Binance and Bitfinex RSI and VWAP calculator. It is implemented both in Python and Rust. Full task text can be found in [TASK.md](TASK.md)


## Launching

### Python
Make sure you have installed Python with version >= 3.10
Install dependencies
```sh
cd python
pip install -r requirements.txt
chmod +x ./src/main.py
./src/main.py
```

### Rust
Make sure you have Rust installed.
```sh
cd rust
cargo build --release
./target/release/test-task
```
