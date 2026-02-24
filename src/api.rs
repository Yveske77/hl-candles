use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};

use crate::cache::Cache;
use crate::types::{HealthResponse, SymbolResponse};

pub async fn handle_get_all_candles(
    State(cache): State<Cache>,
) -> impl IntoResponse {
    let all_candles = cache.get_all().await;
    let last_update = cache.get_last_update().await;

    let mut headers = HeaderMap::new();
    if let Some(time) = last_update {
        if let Ok(etag) = format!("\"{}\"", time.timestamp()).parse() {
            headers.insert("ETag", etag);
        }
    }

    (headers, Json(all_candles))
}

pub async fn handle_get_symbol_candles(
    Path(symbol): Path<String>,
    State(cache): State<Cache>,
) -> impl IntoResponse {
    let symbol_upper = symbol.to_uppercase();
    
    if symbol_upper.is_empty() {
        return (StatusCode::BAD_REQUEST, "Symbol required").into_response();
    }

    if let Some(entry) = cache.get(&symbol_upper).await {
        let mut headers = HeaderMap::new();
        if let Ok(etag) = format!("\"{}\"", entry.last_update.timestamp()).parse() {
            headers.insert("ETag", etag);
        }
        (headers, Json(entry)).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Symbol not found").into_response()
    }
}

pub async fn handle_get_symbols(
    State(cache): State<Cache>,
) -> impl IntoResponse {
    let symbols = cache.get_symbols().await;
    let count = symbols.len();

    let response = SymbolResponse {
        symbols,
        count,
    };

    Json(response).into_response()
}

pub async fn handle_health(
    State(cache): State<Cache>,
) -> impl IntoResponse {
    let symbols = cache.get_symbols().await;
    let last_update = cache.get_last_update().await;
    let symbol_update = cache.get_symbol_update().await;

    let health = HealthResponse {
        status: "healthy".to_string(),
        symbol_count: symbols.len(),
        last_update,
        symbol_update,
    };

    Json(health).into_response()
}
