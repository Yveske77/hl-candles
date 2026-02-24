use std::time::Duration;
use reqwest::Client;
use crate::types::{Candle, HyperliquidCandle, HyperliquidCandleRequest, HyperliquidCandleRequestData, MetaRequest, MetaResponse};

const HYPERLIQUID_URL: &str = "https://api.hyperliquid.xyz/info";

#[derive(Clone)]
pub struct ApiClient {
    http_client: Client,
    _api_key: String, // Kept here just like original hydromancer client
}

impl ApiClient {
    pub fn new(api_key: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build reqwest client");
            
        Self {
            http_client: client,
            _api_key: api_key,
        }
    }

    // Fetches perpetual symbols via Hyperliquid's meta endpoint
    pub async fn fetch_perpetual_symbols(&self) -> Result<Vec<String>, String> {
        let req_body = MetaRequest {
            typ: "meta".to_string(),
        };

        let resp = self.http_client
            .post(HYPERLIQUID_URL)
            .json(&req_body)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("API returned status {}: {}", status, body));
        }

        let meta_resp: MetaResponse = resp.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let symbols: Vec<String> = meta_resp.universe
            .into_iter()
            .filter(|item| !item.name.is_empty() && !item.is_delisted)
            .map(|item| item.name)
            .collect();

        Ok(symbols)
    }

    pub async fn fetch_candles(
        &self,
        symbol: &str,
        interval: &str,
        start_time: i64,
        end_time: i64,
    ) -> Result<Vec<Candle>, String> {
        let req_body = HyperliquidCandleRequest {
            typ: "candleSnapshot".to_string(),
            req: HyperliquidCandleRequestData {
                coin: symbol.to_string(),
                interval: interval.to_string(),
                start_time,
                end_time,
            },
        };

        let resp = self.http_client
            .post(HYPERLIQUID_URL)
            .json(&req_body)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("API returned status {}: {}", status, body));
        }

        let raw_candles: Vec<HyperliquidCandle> = resp.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let candles = raw_candles.into_iter().map(|rc| Candle {
            timestamp: rc.t,
            open: rc.o,
            high: rc.h,
            low: rc.l,
            close: rc.c,
            volume: rc.v,
        }).collect();

        Ok(candles)
    }

    pub async fn fetch_candles_with_retry(
        &self,
        symbol: &str,
        interval: &str,
        start_time: i64,
        end_time: i64,
        max_retries: u32,
    ) -> Result<Vec<Candle>, String> {
        let mut last_err = String::new();

        for attempt in 0..max_retries {
            match self.fetch_candles(symbol, interval, start_time, end_time).await {
                Ok(candles) => return Ok(candles),
                Err(e) => {
                    last_err = e;
                    if attempt < max_retries - 1 {
                        let backoff = Duration::from_secs(1 << attempt);
                        tokio::time::sleep(backoff).await;
                    }
                }
            }
        }

        Err(format!("Failed after {} retries: {}", max_retries, last_err))
    }
}
