# 🚀 API Usage Guide

Your Hyperliquid backend is live at:
**https://backend-hyperliquid-candles-production.up.railway.app**

---

## ✅ API Status

**Current Status:** 🟢 HEALTHY

- ✅ 184 active perpetual pairs
- ✅ 169 hourly candles per symbol (7 days)
- ✅ Auto-refreshes every 5 minutes
- ✅ CORS enabled for all origins

**Latest Prices:**
- BTC: $96,027.00
- ETH: $3,209.80

---

## 📡 API Endpoints

### 1. Health Check
**GET** `/health`

Returns server status and statistics.

**Example:**
```bash
curl https://backend-hyperliquid-candles-production.up.railway.app/health
```

**Response:**
```json
{
  "status": "healthy",
  "symbol_count": 184,
  "last_update": "2025-11-15T18:32:00Z",
  "symbol_update": "2025-11-15T18:31:35Z"
}
```

---

### 2. Get All Symbols
**GET** `/api/symbols`

Returns list of all available trading pairs.

**Example:**
```bash
curl https://backend-hyperliquid-candles-production.up.railway.app/api/symbols
```

**Response:**
```json
{
  "symbols": ["BTC", "ETH", "SOL", "AVAX", ...],
  "count": 184
}
```

---

### 3. Get Candles for Specific Symbol
**GET** `/api/candles/:symbol`

Returns 7 days of hourly candle data for a specific symbol.

**Examples:**
```bash
# BTC candles
curl https://backend-hyperliquid-candles-production.up.railway.app/api/candles/BTC

# ETH candles
curl https://backend-hyperliquid-candles-production.up.railway.app/api/candles/ETH

# SOL candles
curl https://backend-hyperliquid-candles-production.up.railway.app/api/candles/SOL
```

**Response:**
```json
{
  "symbol": "BTC",
  "candles": [
    {
      "timestamp": 1731672000000,
      "open": 96219.0,
      "high": 96353.0,
      "low": 95957.0,
      "close": 96027.0,
      "volume": 277.88
    },
    ...
  ],
  "last_update": "2025-11-15T18:32:00Z"
}
```

---

### 4. Get All Candles (Full Cache)
**GET** `/api/candles`

Returns candle data for ALL symbols at once (large response, ~2.8MB).

**Example:**
```bash
curl https://backend-hyperliquid-candles-production.up.railway.app/api/candles
```

**Response:**
```json
{
  "BTC": {
    "symbol": "BTC",
    "candles": [...],
    "last_update": "2025-11-15T18:32:00Z"
  },
  "ETH": {
    "symbol": "ETH",
    "candles": [...],
    "last_update": "2025-11-15T18:32:00Z"
  },
  ...
}
```

**Tip:** Use gzip compression for this endpoint:
```bash
curl -H "Accept-Encoding: gzip" https://backend-hyperliquid-candles-production.up.railway.app/api/candles --compressed
```

---

## 💻 Usage Examples

### JavaScript / Node.js

```javascript
// Fetch BTC candles
fetch('https://backend-hyperliquid-candles-production.up.railway.app/api/candles/BTC')
  .then(response => response.json())
  .then(data => {
    console.log(`BTC: ${data.candles.length} candles`);
    const latest = data.candles[data.candles.length - 1];
    console.log(`Latest price: $${latest.close}`);
  });

// Get all symbols
fetch('https://backend-hyperliquid-candles-production.up.railway.app/api/symbols')
  .then(response => response.json())
  .then(data => {
    console.log(`Total symbols: ${data.count}`);
    console.log(`Symbols: ${data.symbols.join(', ')}`);
  });
```

### Python

```python
import requests

# Base URL
BASE_URL = "https://backend-hyperliquid-candles-production.up.railway.app"

# Get BTC candles
response = requests.get(f"{BASE_URL}/api/candles/BTC")
data = response.json()
print(f"BTC: {len(data['candles'])} candles")
print(f"Latest price: ${data['candles'][-1]['close']:,.2f}")

# Get all symbols
response = requests.get(f"{BASE_URL}/api/symbols")
symbols = response.json()
print(f"Total symbols: {symbols['count']}")
print(f"First 10: {', '.join(symbols['symbols'][:10])}")

# Get health status
response = requests.get(f"{BASE_URL}/health")
health = response.json()
print(f"Status: {health['status']}")
print(f"Symbol count: {health['symbol_count']}")
```

### React / Next.js

```typescript
'use client';

import { useEffect, useState } from 'react';

const BASE_URL = 'https://backend-hyperliquid-candles-production.up.railway.app';

interface Candle {
  timestamp: number;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

interface CandleData {
  symbol: string;
  candles: Candle[];
  last_update: string;
}

export default function BTCPrice() {
  const [data, setData] = useState<CandleData | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch(`${BASE_URL}/api/candles/BTC`)
      .then(res => res.json())
      .then(data => {
        setData(data);
        setLoading(false);
      });
  }, []);

  if (loading) return <div>Loading...</div>;
  if (!data) return <div>No data</div>;

  const latestCandle = data.candles[data.candles.length - 1];

  return (
    <div>
      <h1>BTC Price</h1>
      <p>Latest: ${latestCandle.close.toLocaleString()}</p>
      <p>24h High: ${latestCandle.high.toLocaleString()}</p>
      <p>24h Low: ${latestCandle.low.toLocaleString()}</p>
      <p>Volume: {latestCandle.volume.toFixed(2)} BTC</p>
    </div>
  );
}
```

### cURL Examples

```bash
# Health check
curl https://backend-hyperliquid-candles-production.up.railway.app/health

# Get symbols (pretty print)
curl https://backend-hyperliquid-candles-production.up.railway.app/api/symbols | jq

# Get BTC candles (pretty print)
curl https://backend-hyperliquid-candles-production.up.railway.app/api/candles/BTC | jq

# Get latest BTC price only
curl -s https://backend-hyperliquid-candles-production.up.railway.app/api/candles/BTC | jq '.candles[-1].close'

# Get multiple symbols in parallel
curl -s https://backend-hyperliquid-candles-production.up.railway.app/api/candles/BTC & \
curl -s https://backend-hyperliquid-candles-production.up.railway.app/api/candles/ETH & \
curl -s https://backend-hyperliquid-candles-production.up.railway.app/api/candles/SOL & \
wait

# Save BTC data to file
curl https://backend-hyperliquid-candles-production.up.railway.app/api/candles/BTC -o btc_candles.json

# Get compressed full cache
curl -H "Accept-Encoding: gzip" \
  https://backend-hyperliquid-candles-production.up.railway.app/api/candles \
  --compressed -o all_candles.json
```

### Go

```go
package main

import (
    "encoding/json"
    "fmt"
    "io"
    "net/http"
)

const BaseURL = "https://backend-hyperliquid-candles-production.up.railway.app"

type Candle struct {
    Timestamp int64   `json:"timestamp"`
    Open      float64 `json:"open"`
    High      float64 `json:"high"`
    Low       float64 `json:"low"`
    Close     float64 `json:"close"`
    Volume    float64 `json:"volume"`
}

type CandleData struct {
    Symbol     string   `json:"symbol"`
    Candles    []Candle `json:"candles"`
    LastUpdate string   `json:"last_update"`
}

func main() {
    // Get BTC candles
    resp, err := http.Get(BaseURL + "/api/candles/BTC")
    if err != nil {
        panic(err)
    }
    defer resp.Body.Close()

    body, _ := io.ReadAll(resp.Body)
    
    var data CandleData
    json.Unmarshal(body, &data)
    
    fmt.Printf("BTC: %d candles\n", len(data.Candles))
    latest := data.Candles[len(data.Candles)-1]
    fmt.Printf("Latest price: $%.2f\n", latest.Close)
}
```

---

## 📊 Data Structure

### Candle Object
```typescript
{
  timestamp: number;  // Unix timestamp in milliseconds
  open: number;       // Opening price
  high: number;       // Highest price in period
  low: number;        // Lowest price in period
  close: number;      // Closing price
  volume: number;     // Trading volume
}
```

### Available Symbols (184 total)
Major pairs: BTC, ETH, SOL, AVAX, BNB, ATOM, DYDX, APE, OP, LTC, ARB, DOGE, INJ, SUI, kPEPE, CRV, LDO, LINK, STX, CFX, and 164 more...

---

## ⚡ Performance

- **Response Time:** <50ms for single symbol
- **Response Time:** <500ms for all symbols (gzipped)
- **Data Freshness:** Updates every 5 minutes
- **Symbol List:** Updates every 60 minutes
- **Uptime:** Monitored by Railway health checks

---

## 🔒 Security

- ✅ CORS enabled for all origins
- ✅ No authentication required (public API)
- ✅ Rate limiting handled gracefully
- ✅ HTTPS enabled by default

---

## 🐛 Error Handling

### 404 Not Found
```bash
curl https://backend-hyperliquid-candles-production.up.railway.app/api/candles/INVALID
# Returns: "Symbol not found"
```

### Empty Candle Arrays
Some symbols may return empty candle arrays due to rate limiting. They will be retried automatically every 5 minutes.

```json
{
  "symbol": "EXAMPLE",
  "candles": [],
  "last_update": "2025-11-15T18:32:00Z"
}
```

---

## 💡 Use Cases

### 1. Price Monitoring Dashboard
Create a real-time dashboard showing prices for multiple pairs:
```javascript
const symbols = ['BTC', 'ETH', 'SOL', 'AVAX'];
symbols.forEach(async symbol => {
  const res = await fetch(`${BASE_URL}/api/candles/${symbol}`);
  const data = await res.json();
  const price = data.candles[data.candles.length - 1].close;
  console.log(`${symbol}: $${price}`);
});
```

### 2. Trading Bot
Use candle data for technical analysis:
```python
def calculate_sma(candles, period=20):
    closes = [c['close'] for c in candles[-period:]]
    return sum(closes) / len(closes)

response = requests.get(f"{BASE_URL}/api/candles/BTC")
candles = response.json()['candles']
sma_20 = calculate_sma(candles, 20)
print(f"BTC 20-period SMA: ${sma_20:,.2f}")
```

### 3. Historical Analysis
Analyze price movements over 7 days:
```python
import pandas as pd

response = requests.get(f"{BASE_URL}/api/candles/BTC")
candles = response.json()['candles']

df = pd.DataFrame(candles)
df['datetime'] = pd.to_datetime(df['timestamp'], unit='ms')

print(f"7-day high: ${df['high'].max():,.2f}")
print(f"7-day low: ${df['low'].min():,.2f}")
print(f"Average volume: {df['volume'].mean():.2f}")
```

### 4. Multi-Pair Comparison
Compare performance across multiple pairs:
```javascript
async function compareSymbols(symbols) {
  const promises = symbols.map(symbol =>
    fetch(`${BASE_URL}/api/candles/${symbol}`).then(r => r.json())
  );
  
  const results = await Promise.all(promises);
  
  results.forEach(data => {
    const first = data.candles[0];
    const last = data.candles[data.candles.length - 1];
    const change = ((last.close - first.close) / first.close) * 100;
    console.log(`${data.symbol}: ${change > 0 ? '+' : ''}${change.toFixed(2)}%`);
  });
}

compareSymbols(['BTC', 'ETH', 'SOL', 'AVAX']);
```

---

## 📈 Monitoring

### Check if API is up
```bash
curl https://backend-hyperliquid-candles-production.up.railway.app/health
```

### Monitor in your app
```javascript
setInterval(async () => {
  const res = await fetch(`${BASE_URL}/health`);
  const health = await res.json();
  console.log(`Status: ${health.status}, Symbols: ${health.symbol_count}`);
}, 60000); // Check every minute
```

---

## 🔗 Links

- **API Base URL:** https://backend-hyperliquid-candles-production.up.railway.app
- **GitHub Repo:** https://github.com/luckshury/backend-hyperliquid-candles
- **Railway Dashboard:** https://railway.app (view logs, metrics, scale)

---

## ❓ FAQ

**Q: How often is the data updated?**  
A: Candles refresh every 5 minutes, symbols refresh every 60 minutes.

**Q: How many days of history?**  
A: 7 days of hourly candle data (169 candles per symbol).

**Q: Is there a rate limit?**  
A: No rate limit on your end. The backend handles Hyperliquid's rate limits internally.

**Q: Can I use this in production?**  
A: Yes! It's production-ready with error handling, retries, and health checks.

**Q: What if a symbol has no candles?**  
A: Empty arrays are normal for rate-limited symbols. They retry automatically every 5 minutes.

**Q: Can I request more history?**  
A: Update the `CANDLE_DAYS` environment variable in Railway (default: 7).

**Q: Can I change the interval?**  
A: Update the `CANDLE_INTERVAL` environment variable (options: 1m, 5m, 15m, 1h, 4h, 1d).

---

## 🚀 Next Steps

1. **Build a dashboard** using the API
2. **Create a trading bot** with the candle data
3. **Add more features** to the backend (fork the repo!)
4. **Monitor performance** in Railway dashboard
5. **Scale up** if needed (Railway auto-scales)

---

**Happy Trading! 📈**

