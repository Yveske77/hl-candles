use std::time::Duration;
use tokio::time;
use tracing::{error, info, warn};

use crate::cache::Cache;
use crate::clients::ApiClient;

pub async fn symbol_fetcher_task(
    cache: Cache,
    api_client: ApiClient,
    refresh_interval: Duration,
) {
    info!("[SymbolFetcher] Task started");
    let mut cached_symbols: Vec<String> = Vec::new();

    loop {
        info!("[SymbolFetcher] Fetching perpetual symbols from Hyperliquid...");
        match api_client.fetch_perpetual_symbols().await {
            Ok(symbols) => {
                if symbols.is_empty() {
                    warn!("[SymbolFetcher] WARNING: Received empty symbol list");
                } else {
                    info!("[SymbolFetcher] Discovered {} symbols from Hyperliquid", symbols.len());
                    cache.set_symbols(symbols.clone()).await;
                    cached_symbols = symbols;
                }
            }
            Err(e) => {
                error!("[SymbolFetcher] ERROR: Failed to fetch symbols: {}", e);
                if !cached_symbols.is_empty() {
                    info!("[SymbolFetcher] Using cached symbol list ({} symbols)", cached_symbols.len());
                    cache.set_symbols(cached_symbols.clone()).await;
                }
            }
        }
        time::sleep(refresh_interval).await;
    }
}

pub async fn candle_fetcher_task(
    cache: Cache,
    api_client: ApiClient,
    refresh_interval: Duration,
    candle_interval: String,
    candle_days: i64,
) {
    info!("[CandleFetcher] Task started");
    let batch_size = 10;
    let batch_delay = Duration::from_millis(200);

    loop {
        let symbols = cache.get_symbols().await;

        if symbols.is_empty() {
            info!("[CandleFetcher] No symbols available yet, skipping fetch");
            time::sleep(Duration::from_secs(5)).await;
            continue;
        }

        info!("[CandleFetcher] Found {} symbols, starting candle fetch...", symbols.len());

        let end_time = chrono::Utc::now().timestamp_millis();
        let start_time = chrono::Utc::now().timestamp_millis() - (candle_days * 24 * 60 * 60 * 1000);

        let total_batches = (symbols.len() + batch_size - 1) / batch_size;
        let mut success_count = 0;

        for (batch_idx, chunk) in symbols.chunks(batch_size).enumerate() {
            let current_batch = batch_idx + 1;
            info!("[CandleFetcher] Fetching batch {}/{} ({} symbols)...", current_batch, total_batches, chunk.len());

            let mut iter_handles = Vec::new();

            for symbol in chunk {
                let api_client = api_client.clone();
                let cache = cache.clone();
                let sym = symbol.clone();
                let interval = candle_interval.clone();

                let handle = tokio::spawn(async move {
                    let res = api_client.fetch_candles_with_retry(&sym, &interval, start_time, end_time, 3).await;
                    match res {
                        Ok(candles) => {
                            cache.set(sym.clone(), candles).await;
                            Ok(())
                        }
                        Err(e) => {
                            error!("[CandleFetcher] ERROR: Failed to fetch {}: {}", sym, e);
                            cache.set(sym.clone(), Vec::new()).await;
                            Err(())
                        }
                    }
                });
                iter_handles.push(handle);
            }

            for handle in iter_handles {
                if let Ok(Ok(_)) = handle.await {
                    success_count += 1;
                }
            }

            if current_batch < total_batches {
                time::sleep(batch_delay).await;
            }
        }

        info!("[CandleFetcher] Batch {}/{} complete ({} symbols cached successfully)", total_batches, total_batches, success_count);
        info!("[CandleFetcher] ✓ Cached {}/{} symbols", success_count, symbols.len());

        time::sleep(refresh_interval).await;
    }
}
