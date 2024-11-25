# Token Price Oracle/Service

The Token Price Oracle Service is a high-performance, asynchronous API designed to provide accurate and real-time token price data. This service integrates seamlessly with multiple data sources, ensuring reliability, scalability, and flexibility for developers building applications that require up-to-date cryptocurrency price information.

## Key Features

 1. Asynchronous API
    The service is built using modern asynchronous Rust libraries to handle high-concurrency requests efficiently.
    Users can fetch token prices without blocking, allowing for smooth integration into distributed systems or microservice architectures.

2. Integration with Multiple Data Sources
    Connects to multiple cryptocurrency exchanges, market aggregators, and data providers to fetch live and historical price data.
    Utilizes redundancy by querying multiple sources, ensuring high availability and accuracy even if one provider is down.

3. Real-Time Price Updates
    Supports WebSocket and HTTP APIs for continuous price feeds or on-demand queries.
    Ensures low latency in delivering updates to keep your application in sync with market changes.

4. Advanced Caching System
    Implements an intelligent in-memory caching layer to handle a large number of concurrent readers efficiently.
    Reduces the number of external API calls to data providers, significantly cutting operational costs.
    Configurable cache expiration to balance data freshness with performance.
    Particularly beneficial for applications with heavy read traffic, ensuring high performance while minimizing API provider expenses.

5.  Customizable Aggregation Logic
    Offers configurable algorithms for price aggregation (e.g., weighted averages, median pricing).
    Allows developers to fine-tune how prices are calculated from different data sources based on their application's requirements.

6. Secure and Scalable
    Implements secure communication channels (e.g., HTTPS, token-based authentication) for API usage.
    Scales horizontally to handle a growing number of requests and maintain performance under heavy loads.


## Example Use Cases

1. DeFi Applications

   Fetch real-time prices for on-chain token swaps, lending, or liquidity pool management.

2. Portfolio Management

    Provide users with live token valuations and historical price trends.

3. Analytics Dashboards

    Aggregate and visualize token price movements for traders and investors.

4. High-Read Workloads

    Use the advanced caching system to efficiently serve thousands of simultaneous requests while minimizing costs.

## API Overview

### HTTP Endpoints

    GET /price/{token}: Fetch the latest price for a specific token.
    GET /prices Fetch the latest price for all token.

### WebSocket

Subscribe to real-time price updates with a customizable feed (e.g., specific tokens or exchanges).

### Response Format

All responses are returned in JSON with consistent, developer-friendly formats.

```json
{
"bitcoin": {
"usd": 96000,
"eur": 92148
},
"ethereum": {
"usd": 3312.2,
"eur": 3179.31
}
}
```


## Benefits of the Service

1. Accuracy: By aggregating data from multiple reliable sources, the service minimizes the risk of price manipulation or inaccuracies.
2. Efficiency: Built-in caching and asynchronous processing enable high throughput and low response times.
3. Cost Savings: The advanced caching system optimizes for high-read scenarios, reducing the reliance on external APIs and significantly lowering costs for frequent access.
4. Flexibility: Easy integration into applications of varying scale and complexity.
5. Scalability: Designed to grow with your needs, handling thousands of concurrent users and API calls.

The Token Price Oracle Service is your dependable solution for accessing accurate, real-time cryptocurrency price data while minimizing costs for high-demand applications.


## Road Map

--> 1. MVP Solution. Getting price data for btc/eth tokens from the 1 source (coingeko). Store data only in the runtime custom cash.
2. Add ability to configur list of tokens, coingeko keys, time intervals in the config file. Add unit tests.
3. Add light weight DB to stor data and provide historical price data.
4. Add ability to work with multiple proxies to bypass request limits. 
5. Integrate more data sources



