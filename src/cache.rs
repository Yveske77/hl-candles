use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

use crate::types::{CacheEntry, Candle};

#[derive(Debug, Default)]
struct CacheData {
    data: HashMap<String, CacheEntry>,
    symbols: Vec<String>,
    last_update: Option<DateTime<Utc>>,
    symbol_update: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct Cache {
    inner: Arc<RwLock<CacheData>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(CacheData::default())),
        }
    }

    pub async fn set(&self, symbol: String, candles: Vec<Candle>) {
        let mut guard = self.inner.write().await;
        let now = Utc::now();
        guard.data.insert(
            symbol.clone(),
            CacheEntry {
                symbol,
                candles,
                last_update: now,
            },
        );
        guard.last_update = Some(now);
    }

    pub async fn get(&self, symbol: &str) -> Option<CacheEntry> {
        let guard = self.inner.read().await;
        guard.data.get(symbol).cloned()
    }

    pub async fn get_all(&self) -> HashMap<String, CacheEntry> {
        let guard = self.inner.read().await;
        // Returning a cloned hashmap to emulate the pass-by-value / thread-safe approach of original Go Code
        guard.data.clone()
    }

    pub async fn set_symbols(&self, symbols: Vec<String>) {
        let mut guard = self.inner.write().await;
        guard.symbols = symbols;
        guard.symbol_update = Some(Utc::now());
    }

    pub async fn get_symbols(&self) -> Vec<String> {
        let guard = self.inner.read().await;
        guard.symbols.clone()
    }

    pub async fn get_last_update(&self) -> Option<DateTime<Utc>> {
        let guard = self.inner.read().await;
        guard.last_update
    }

    pub async fn get_symbol_update(&self) -> Option<DateTime<Utc>> {
        let guard = self.inner.read().await;
        guard.symbol_update
    }
}
