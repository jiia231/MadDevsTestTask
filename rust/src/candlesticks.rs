use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct BinanceCandlestickData {
    // Candlestick details
    pub k: BinanceCandlestick,
}

#[derive(Deserialize)]
pub struct BinanceCandlestick {
    // Kline start time
    #[serde(rename = "t")]
    #[serde(deserialize_with = "parse_u64_to_datetime")]
    pub st: DateTime<Utc>,

    // Kline close time
    #[serde(rename = "T")]
    #[serde(deserialize_with = "parse_u64_to_datetime")]
    pub ct: DateTime<Utc>,

    // Symbol
    pub s: String,

    // Interval
    pub i: String,

    // First trade ID
    #[serde(rename = "f")]
    pub ft: u64,

    // Last trade ID
    #[serde(rename = "L")]
    pub lt: u64,

    // Open price
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub o: f64,

    // Close price
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub c: f64,

    // High price
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub h: f64,

    // Low price
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub l: f64,

    // Base asset volume
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub v: f64,

    // Number of trades
    pub n: u64,

    // Is this kline closed?
    pub x: bool,

    // Quote asset volume
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub q: f64,

    // Taker buy base asset volume
    #[serde(rename = "V")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub bv: f64,

    // Taker buy quote asset volume
    #[serde(rename = "Q")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub qv: f64,
}

fn parse_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    // Parse the string into an f64
    let parsed_value = s.parse::<f64>().map_err(serde::de::Error::custom)?;

    Ok(parsed_value)
}

fn parse_u64_to_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let ts: u64 = Deserialize::deserialize(deserializer)?;

    // Parse the string into an f64
    let parsed_value = Utc.timestamp_millis_opt(ts as i64).unwrap();

    Ok(parsed_value)
}

#[derive(Clone, Deserialize)]
pub struct BitfinexCandlestick {
    // The candle data is represented as an array with elements in a specific order.
    #[serde(deserialize_with = "parse_u64_to_datetime")]
    pub mts: DateTime<Utc>,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}

#[derive(Clone, Deserialize)]
pub struct BitfinexCandlestickSnapshotData {
    pub channel_id: i64,
    pub candle_data: Vec<BitfinexCandlestick>,
}

#[derive(Clone, Deserialize)]
pub struct BitfinexCandlestickUpdateData {
    pub channel_id: i64,
    pub candle_data: BitfinexCandlestick,
}
