mod api;
mod cache;
mod clients;
mod types;
mod workers;

use axum::{
    routing::get,
    Router,
};
use std::env;
use std::time::Duration;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{info, Level};

use crate::cache::Cache;
use crate::clients::ApiClient;
use crate::workers::{candle_fetcher_task, symbol_fetcher_task};

#[tokio::main]
async fn main() {
    // Attempt to load .env, ignore if missing
    let _ = dotenvy::dotenv();

    // Setup logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let hydromancer_api_key = env::var("HYDROMANCER_API_KEY")
        .unwrap_or_else(|_| "sk_nNhuLkdGdW5sxnYec33C2FBPzLjXBnEd".to_string());
    let candle_interval = env::var("CANDLE_INTERVAL").unwrap_or_else(|_| "1h".to_string());
    let candle_days: i64 = env::var("CANDLE_DAYS")
        .unwrap_or_else(|_| "7".to_string())
        .parse()
        .unwrap_or(7);
    let refresh_interval_min: u64 = env::var("REFRESH_INTERVAL_MIN")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .unwrap_or(5);
    let symbol_refresh_interval_min: u64 = env::var("SYMBOL_REFRESH_INTERVAL_MIN")
        .unwrap_or_else(|_| "60".to_string())
        .parse()
        .unwrap_or(60);

    // Initialize shared cache and client
    let cache = Cache::new();
    let api_client = ApiClient::new(hydromancer_api_key);

    // Spawn symbol fetcher task (Actor equivalent)
    let symbol_cache = cache.clone();
    let symbol_client = api_client.clone();
    tokio::spawn(async move {
        symbol_fetcher_task(
            symbol_cache,
            symbol_client,
            Duration::from_secs(symbol_refresh_interval_min * 60),
        )
        .await;
    });

    // Spawn candle fetcher task (Actor equivalent)
    let candle_cache = cache.clone();
    let candle_client = api_client.clone();
    let candle_interval_clone = candle_interval.clone();
    tokio::spawn(async move {
        candle_fetcher_task(
            candle_cache,
            candle_client,
            Duration::from_secs(refresh_interval_min * 60),
            candle_interval_clone,
            candle_days,
        )
        .await;
    });

    // Configure CORS and Compression
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/candles", get(api::handle_get_all_candles))
        .route("/api/candles/:symbol", get(api::handle_get_symbol_candles))
        .route("/api/symbols", get(api::handle_get_symbols))
        .route("/health", get(api::handle_health))
        .with_state(cache)
        .layer(CompressionLayer::new())
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr = format!("0.0.0.0:{}", port);
    info!("Server started on port {}", port);
    info!("Candle interval: {}, History: {} days", candle_interval, candle_days);
    info!(
        "Refresh intervals - Candles: {}m, Symbols: {}m",
        refresh_interval_min, symbol_refresh_interval_min
    );

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
