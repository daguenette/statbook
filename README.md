# Statbook

[![CI](https://github.com/daguenette/statbook/workflows/CI/badge.svg)](https://github.com/daguenette/statbook/actions)
[![Crates.io](https://img.shields.io/crates/v/statbook.svg)](https://crates.io/crates/statbook)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/daguenette/statbook#license)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

(Early in development)
<!--toc:start-->
- [Statbook](#statbook)
  - [Features](#features)
  - [Setup](#setup)
    - [1. Get API Credentials](#1-get-api-credentials)
    - [2. Add to Cargo.toml](#2-add-to-cargotoml)
    - [3. Set Environment Variables](#3-set-environment-variables)
  - [Quick Start](#quick-start)
  - [Configuration Options](#configuration-options)
    - [Environment Variables (Recommended)](#environment-variables-recommended)
    - [Custom Configuration with Builder Pattern](#custom-configuration-with-builder-pattern)
    - [Direct Configuration](#direct-configuration)
  - [API Reference](#api-reference)
    - [Core Functions](#core-functions)
    - [Data Types](#data-types)
    - [Fetch Strategies](#fetch-strategies)
  - [Error Handling](#error-handling)
    - [Partial Failure Handling](#partial-failure-handling)
  - [Testing](#testing)
    - [Unit Testing with Mock Providers](#unit-testing-with-mock-providers)
    - [Custom Mock Data](#custom-mock-data)
    - [Integration Testing](#integration-testing)
    - [Running Tests](#running-tests)
  - [Advanced Usage](#advanced-usage)
    - [Custom Providers](#custom-providers)
  - [Features](#features)
  - [Future Plans](#future-plans)
  - [License](#license)
<!--toc:end-->

A high-performance Rust library for accessing sports statistics and news data
with concurrent API calls, comprehensive error handling, and flexible
configuration options.

**Perfect for fantasy sports apps, sports analytics, news aggregation,
and data-driven sports applications.**

Currently supports NFL player data via [MySportsFeeds.com](https://www.mysportsfeeds.com)
and news data via [NewsAPI.org](https://newsapi.org) with plans
to expand to other sports and data sources.

## Features

- **Flexible configuration** with builder pattern and environment variables
- **Multiple fetch strategies** (stats-only, news-only, or both)
- **Comprehensive error handling** with detailed error types
- **Built-in testing utilities** with mock providers
- **Extensible architecture** with trait-based providers
- **Concurrent API calls** for improved performance
- **Clean API** with intuitive imports and type safety

## Setup

### 1. Get API Credentials

1. Sign up for a free account at [MySportsFeeds.com](https://www.mysportsfeeds.com)
   to get your stats API key
2. Sign up for a free account at [NewsAPI.org](https://newsapi.org)
   to get your news API key

### 2. Add to Cargo.toml

```toml
[dependencies]
statbook = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### 3. Set Environment Variables

```bash
export STATS_API_KEY="your-mysportsfeeds-api-key"
export NEWS_API_KEY="your-newsapi-key"
```

## Quick Start

```rust
use statbook::{StatbookClient, FetchStrategy, api::players::get_player_summary};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variables
    let client = StatbookClient::from_env()?;

    // Fetch player data with concurrent API calls
    let result = get_player_summary(&client, "josh-allen", 
        FetchStrategy::Both { fail_on_news_error: false }).await?;

    let stats = &result.player_stats;
    println!("{} {} - {} #{}", 
        stats.first_name, 
        stats.last_name, 
        stats.primary_position,
        stats.jersey_number
    );
    println!("Team: {} | Games: {}", stats.current_team, stats.games_played);

    // Handle news with graceful error handling
    match result.news_result {
        Ok(articles) => {
            println!("Recent News ({} articles):", articles.len());
            for article in articles.iter().take(2) {
                println!("  • {}", article.title);
            }
        }
        Err(e) => println!("News unavailable: {}", e),
    }

    Ok(())
}
```

## Configuration Options

### Environment Variables (Recommended)

```rust
use statbook::StatbookClient;

let client = StatbookClient::from_env()?;
```

### Custom Configuration with Builder Pattern

```rust
use statbook::{StatbookClient, StatbookConfig, NewsConfig, SortBy};

// Custom news settings
let news_config = NewsConfig::new()
    .with_max_articles(15)
    .with_days_back(30)
    .with_sort_by(SortBy::Relevancy);

let config = StatbookConfig::builder()
    .stats_api_key("your-mysportsfeeds-api-key")
    .news_api_key("your-newsapi-key")
    .news_config(news_config)
    .build()?;

let client = StatbookClient::new(config);
```

### Direct Configuration

```rust
use statbook::{StatbookClient, StatbookConfig};

let config = StatbookConfig::new(
    "your-mysportsfeeds-api-key".to_string(), 
    "your-newsapi-key".to_string()
);
let client = StatbookClient::new(config);
```

## API Reference

### Core Functions

```rust
use statbook::{
    StatbookClient, FetchStrategy, NewsQuery,
    api::players::{get_player_stats, get_player_news, get_player_summary}
};

// Get only player statistics (fastest - single API call)
let stats = get_player_stats(&client, "josh-allen").await?;
println!("{} plays {} for {}", stats.first_name, stats.primary_position, stats.current_team);

// Get only news articles with custom query
let query = NewsQuery::for_player("josh-allen")
    .with_page_size(10)
    .with_date_range("2024-01-01".to_string());
let news = get_player_news(&client, &query).await?;

// Get both with concurrent fetching (recommended)
let result = get_player_summary(&client, "josh-allen", 
    FetchStrategy::Both { fail_on_news_error: false }).await?;
```

### Data Types

```rust
use statbook::{PlayerSummary, PlayerStats, Article, NewsQuery, FetchStrategy};

// PlayerSummary - Complete player information
// PlayerStats - Just the statistics
// Article - News article with title, description, content, published_at
// NewsQuery - Configurable news search parameters
// FetchStrategy - Control how data is fetched (StatsOnly, NewsOnly, Both)
```

### Fetch Strategies

Choose the right strategy for your use case:

```rust
use statbook::{FetchStrategy, api::players::get_player_summary};

// Stats only - fastest option (single API call)
let result = get_player_summary(&client, "josh-allen", FetchStrategy::StatsOnly).await?;
// Use case: Live scoreboards, quick player lookups

// News only - for news aggregation
let result = get_player_summary(&client, "josh-allen", FetchStrategy::NewsOnly).await?;
// Use case: Sports news apps, content aggregation

// Both with graceful degradation (recommended)
let result = get_player_summary(&client, "josh-allen", 
    FetchStrategy::Both { fail_on_news_error: false }).await?;
// Use case: Fantasy apps, comprehensive player profiles

// Both with strict error handling
let result = get_player_summary(&client, "josh-allen", 
    FetchStrategy::Both { fail_on_news_error: true }).await?;
// Use case: Critical applications requiring complete data
```

**Performance Comparison:**

- `StatsOnly`: ~200ms (1 API call)
- `NewsOnly`: ~300ms (1 API call)  
- `Both`: ~400ms (2 concurrent API calls)

## Error Handling

The library provides comprehensive error types with detailed context:

```rust
use statbook::{StatbookClient, StatbookError, api::players::get_player_stats};

match get_player_stats(&client, "unknown-player").await {
    Ok(stats) => {
        println!("Found: {} {}", stats.first_name, stats.last_name);
    }
    Err(StatbookError::PlayerNotFound { name }) => {
        println!("No player named '{}'", name);
        // Suggest similar names, check spelling, etc.
    }
    Err(StatbookError::Network(e)) => {
        println!("Network error: {}", e);
        // Retry logic, check connectivity
    }
    Err(StatbookError::StatsApi { status, message }) => {
        match status {
            401 => println!("Invalid API key: {}", message),
            429 => println!("Rate limited: {}", message),
            _ => println!("Stats API error {}: {}", status, message),
        }
    }
    Err(StatbookError::NewsApi { status, message }) => {
        println!("News API error {}: {}", status, message);
    }
    Err(StatbookError::MissingApiKey { key }) => {
        println!("Missing API key: {}. Set environment variable.", key);
    }
    Err(StatbookError::Config(msg)) => {
        println!("Configuration error: {}", msg);
    }
    Err(StatbookError::Validation(msg)) => {
        println!("Validation error: {}", msg);
    }
    Err(e) => println!("Unexpected error: {}", e),
}
```

### Partial Failure Handling

```rust
use statbook::{FetchStrategy, api::players::get_player_summary};

let result = get_player_summary(&client, "josh-allen", 
    FetchStrategy::Both { fail_on_news_error: false }).await?;

println!("Player: {} {}", result.player_stats.first_name, result.player_stats.last_name);

match result.news_result {
    Ok(articles) => println!("Found {} news articles", articles.len()),
    Err(e) => println!("News failed but stats succeeded: {}", e),
}
```

## Testing

The library provides comprehensive testing utilities
for both unit and integration testing:

### Unit Testing with Mock Providers

```rust
use statbook::{create_mock_client, api::players::get_player_stats};

#[tokio::test]
async fn test_player_stats() {
    // Mock client - no real API calls, instant responses
    let client = create_mock_client();
    let stats = get_player_stats(&client, "josh-allen").await.unwrap();

    assert_eq!(stats.first_name, "Josh");
    assert_eq!(stats.last_name, "Allen");
    assert_eq!(stats.primary_position, "QB");
    assert_eq!(stats.current_team, "BUF");
}

#[tokio::test]
async fn test_fetch_strategies() {
    let client = create_mock_client();

    // Test different strategies
    let stats_only = get_player_summary(&client, "josh-allen", FetchStrategy::StatsOnly).await.unwrap();
    let both = get_player_summary(&client, "josh-allen", 
        FetchStrategy::Both { fail_on_news_error: false }).await.unwrap();

    assert_eq!(stats_only.player_stats.first_name, both.player_stats.first_name);
    assert!(both.news_result.is_ok());
}
```

### Custom Mock Data

```rust
use statbook::{create_custom_mock_client, MockStatsProvider, MockNewsProvider, PlayerStats};

#[tokio::test]
async fn test_custom_data() {
    let mut mock_stats = MockStatsProvider::new();
    mock_stats.add_player_stats("custom-player", PlayerStats {
        first_name: "Custom".to_string(),
        last_name: "Player".to_string(),
        // ... other fields
    });

    let client = create_custom_mock_client(mock_stats, MockNewsProvider::new());
    // Test with your custom data
}
```

### Integration Testing

```rust
use statbook::{skip_if_no_credentials, api::players::get_player_stats};

#[tokio::test]
async fn test_real_api() {
    // Skip test if API credentials not available
    let client = match skip_if_no_credentials() {
        Some(client) => client,
        None => {
            println!("Skipping integration test - no API credentials");
            return;
        }
    };
    
    // Test with real API calls
    let stats = get_player_stats(&client, "josh-allen").await.unwrap();
    assert!(!stats.first_name.is_empty());
    assert_eq!(stats.first_name, "Josh");
}
```

### Running Tests

```bash
# Unit tests only (fast, no API calls)
cargo test

# Integration tests (requires API keys)
STATS_API_KEY="your-key" NEWS_API_KEY="your-key" INTEGRATION_TESTS=1 cargo test

# Run specific test
cargo test test_player_stats
```

## Advanced Usage

### Custom Providers

Implement your own data sources:

```rust
use statbook::{StatsProvider, NewsProvider, PlayerStats, Article, NewsQuery, Result, StatbookClient};
use async_trait::async_trait;
use std::sync::Arc;

struct MyCustomStatsProvider;

#[async_trait]
impl StatsProvider for MyCustomStatsProvider {
    async fn fetch_player_stats(&self, name: &str) -> Result<PlayerStats> {
        // Your custom implementation - fetch from your own API, database, etc.
        Ok(PlayerStats {
            first_name: "Custom".to_string(),
            last_name: "Player".to_string(),
            primary_position: "QB".to_string(),
            jersey_number: 1,
            current_team: "CUSTOM".to_string(),
            injury: String::new(),
            rookie: false,
            games_played: 16,
        })
    }
}

struct MyCustomNewsProvider;

#[async_trait]
impl NewsProvider for MyCustomNewsProvider {
    async fn fetch_player_news(&self, query: &NewsQuery) -> Result<Vec<Article>> {
        // Your custom news implementation
        Ok(vec![Article {
            title: format!("Custom news about {}", query.player_name),
            description: "Custom news description".to_string(),
            published_at: "2024-01-01T00:00:00Z".to_string(),
            content: "Custom news content".to_string(),
        }])
    }
}

// Use your custom providers
let client = StatbookClient::with_providers(
    Arc::new(MyCustomStatsProvider),
    Arc::new(MyCustomNewsProvider),
);

// Or mix custom with mock providers for testing
let client = StatbookClient::with_providers(
    Arc::new(MyCustomStatsProvider),
    Arc::new(statbook::MockNewsProvider::new()),
);
```

## Future Plans

- **Caching layer** for improved performance and reduced API calls
- **Enhanced NFL data** (team statistics, game data, season analytics)
- **Advanced news filtering** (sentiment analysis, relevance scoring)
- **Additional sports** (NHL, NBA, MLB, etc.)
- **More data providers** (ESPN, The Athletic, etc.)
- **Real-time updates** via WebSocket connections
- **Data export** (JSON, CSV, database integration)

## License

Licensed under either of

- Apache License, Version 2.0
- MIT license

at your option.
