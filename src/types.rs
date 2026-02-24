use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candle {
    pub timestamp: i64, // Unix timestamp in milliseconds
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheEntry {
    pub symbol: String,
    pub candles: Vec<Candle>,
    pub last_update: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolList {
    pub symbols: Vec<String>,
    pub last_fetch: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HyperliquidCandleRequest {
    #[serde(rename = "type")]
    pub typ: String,
    pub req: HyperliquidCandleRequestData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HyperliquidCandleRequestData {
    pub coin: String,
    pub interval: String,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
}

// Hyperliquid returns values as strings except for timestamp/count
#[derive(Debug, Deserialize)]
pub struct HyperliquidCandle {
    pub t: i64, // Timestamp
    #[serde(deserialize_with = "deserialize_f64_from_str")]
    pub o: f64,
    #[serde(deserialize_with = "deserialize_f64_from_str")]
    pub h: f64,
    #[serde(deserialize_with = "deserialize_f64_from_str")]
    pub l: f64,
    #[serde(deserialize_with = "deserialize_f64_from_str")]
    pub c: f64,
    #[serde(deserialize_with = "deserialize_f64_from_str")]
    pub v: f64,
    pub n: i64, // Number of trades
}

fn deserialize_f64_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

#[derive(Debug, Serialize)]
pub struct MetaRequest {
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct MetaResponse {
    pub universe: Vec<MetaUniverseItem>,
}

#[derive(Debug, Deserialize)]
pub struct MetaUniverseItem {
    pub name: String,
    #[serde(rename = "isDelisted", default)]
    pub is_delisted: bool,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub symbol_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_update: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct SymbolResponse {
    pub symbols: Vec<String>,
    pub count: usize,
}
