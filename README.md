# Statbook

[![CI](https://github.com/daguenette/statbook/workflows/CI/badge.svg)](https://github.com/daguenette/statbook/actions)
[![Crates.io](https://img.shields.io/crates/v/statbook.svg)](https://crates.io/crates/statbook)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/daguenette/statbook#license)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
(Early in development)
<!--toc:start-->
- [Statbook](#statbook)
  - [Features](#features)
  - [Season Support](#season-support)
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
  - [Error Handling](#error-handling)
    - [Graceful Failure Handling](#graceful-failure-handling)
  - [Testing](#testing)
    - [Unit Testing with Mock Providers](#unit-testing-with-mock-providers)
    - [Custom Mock Data](#custom-mock-data)
    - [Integration Testing](#integration-testing)
    - [Running Tests](#running-tests)
  - [Advanced Usage](#advanced-usage)
    - [Custom Providers](#custom-providers)
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

- **Season parameter support** for historical and current data queries
- **Flexible configuration** with builder pattern and environment variables
- **Comprehensive error handling** with detailed error types
- **Built-in testing utilities** with mock providers
- **Extensible architecture** with trait-based providers
- **Concurrent API calls** for improved performance
- **Clean API** with intuitive imports and type safety

## Season Support

Query player statistics for different seasons and time periods:

```rust
use statbook::{StatbookClient, Season, api::players::get_player_stats};

let client = StatbookClient::from_env()?;

// Current regular season
let current = get_player_stats(&client, "josh-allen", None, &Season::Regular).await?;

// Playoff statistics
let playoffs = get_player_stats(&client, "josh-allen", None, &Season::Playoffs).await?;

// Specific year range
let season_2023 = get_player_stats(&client, "josh-allen", Some((2023, 2024)), &Season::Regular).await?;

// Latest available data
let latest = get_player_stats(&client, "josh-allen", None, &Season::Latest).await?;

println!("Season: {}", current.season); // e.g., "regular" or "2023-2024-playoffs"
```

**Available Season Types:**

- `Season::Regular` - Regular season games
- `Season::Playoffs` - Playoff games only  
- `Season::Current` - Current active season
- `Season::Latest` - Most recent available data
- `Season::Upcoming` - Upcoming season data

**Year Range Format:**

- `None` - Uses default season
- `Some((2023, 2024))` - Specific season range, formatted as "2023-2024-regular"

## Setup

### 1. Get API Credentials

1. Sign up for a free account at [MySportsFeeds.com](https://www.mysportsfeeds.com)
   to get your stats API key
2. Sign up for a free account at [NewsAPI.org](https://newsapi.org)
   to get your news API key

### 2. Add to Cargo.toml

```toml
[dependencies]
statbook = "0.0.3"
tokio = { version = "1.0", features = ["full"] }
```

### 3. Set Environment Variables

```bash
export STATS_API_KEY="your-mysportsfeeds-api-key"
export NEWS_API_KEY="your-newsapi-key"
```

## Quick Start

```rust
use statbook::{StatbookClient, Season, api::players::get_player_summary};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variables
    let client = StatbookClient::from_env()?;

    // Fetch player summary with concurrent API calls for current season
    let result = get_player_summary(&client, "josh-allen", None, &Season::Regular).await?;

    println!("{} {} - {} #{}", 
        result.first_name, 
        result.last_name, 
        result.primary_position,
        result.jersey_number
    );
    println!("Team: {} | Games: {}", 
        result.current_team, 
        result.games_played
    );

    // News is handled gracefully - empty vec if failed
    if result.news.is_empty() {
        println!("No news available");
    } else {
        println!("Recent News ({} articles):", result.news.len());
        for article in result.news.iter().take(2) {
            println!("  • {}", article.title);
        }
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
    StatbookClient, NewsQuery, Season,
    api::players::{get_player_stats, get_player_news, get_player_summary}
};

// Get only player statistics (fastest - single API call)
let stats = get_player_stats(&client, "josh-allen", None, &Season::Regular).await?;
println!("{} plays {} for {} (Season: {})", 
    stats.first_name, stats.primary_position, stats.current_team, stats.season);

// Get playoff stats for specific years
let playoff_stats = get_player_stats(&client, "josh-allen", Some((2023, 2024)), &Season::Playoffs).await?;

// Get only news articles with custom query
let query = NewsQuery::for_player("josh-allen")
    .with_page_size(10)
    .with_date_range("2024-01-01".to_string());
let news = get_player_news(&client, &query).await?;

// Get essential player info with news (concurrent fetching)
let summary = get_player_summary(&client, "josh-allen", None, &Season::Regular).await?;
```

### Data Types

```rust
use statbook::{PlayerSummary, PlayerStats, Article, NewsQuery, Season};

// PlayerSummary - Essential player information with news
// PlayerStats - Detailed statistics with season information
// Article - News article with title, description, content, published_at
// NewsQuery - Configurable news search parameters
// Season - Season type enum (Regular, Playoffs, Current, Latest, Upcoming)
```

### Function Overview

The library provides three main functions for different use cases:

```rust
use statbook::{Season, api::players::{get_player_stats, get_player_news, get_player_summary}};

// Detailed player statistics with season information
let stats = get_player_stats(&client, "josh-allen", None, &Season::Regular).await?;
// Returns: PlayerStats with comprehensive player data

// News articles with metadata
let news = get_player_news(&client, &NewsQuery::for_player("josh-allen")).await?;
// Returns: PlayerNews with articles and query metadata

// Essential player info with news (concurrent fetching)
let summary = get_player_summary(&client, "josh-allen", None, &Season::Regular).await?;
// Returns: PlayerSummary with key stats and news articles
```

## Error Handling

The library provides comprehensive error types with detailed context:

```rust
use statbook::{StatbookClient, StatbookError, Season, api::players::get_player_stats};

match get_player_stats(&client, "unknown-player", None, &Season::Regular).await {
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

### Graceful Failure Handling

```rust
use statbook::{Season, api::players::get_player_summary};

let summary = get_player_summary(&client, "josh-allen", None, &Season::Regular).await?;

println!("Player: {} {}", summary.first_name, summary.last_name);

// News failures are handled gracefully - empty vec if failed
if summary.news.is_empty() {
    println!("No news available (may have failed gracefully)");
} else {
    println!("Found {} news articles", summary.news.len());
}
```

## Testing

The library provides comprehensive testing utilities
for both unit and integration testing:

### Unit Testing with Mock Providers

```rust
use statbook::{create_mock_client, Season, api::players::get_player_stats};

#[tokio::test]
async fn test_player_stats() {
    // Mock client - no real API calls, instant responses
    let client = create_mock_client();
    let stats = get_player_stats(&client, "josh-allen", None, &Season::Regular).await.unwrap();

    assert_eq!(stats.first_name, "Josh");
    assert_eq!(stats.last_name, "Allen");
    assert_eq!(stats.primary_position, "QB");
    assert_eq!(stats.current_team, "BUF");
    assert_eq!(stats.season, "regular");
}

#[tokio::test]
async fn test_player_functions() {
    let client = create_mock_client();

    // Test different functions
    let stats = get_player_stats(&client, "josh-allen", None, &Season::Regular).await.unwrap();
    let summary = get_player_summary(&client, "josh-allen", None, &Season::Regular).await.unwrap();

    assert_eq!(stats.first_name, summary.first_name);
    assert!(!summary.news.is_empty());
}

#[tokio::test]
async fn test_season_parameters() {
    let client = create_mock_client();

    // Test different seasons
    let regular = get_player_stats(&client, "josh-allen", None, &Season::Regular).await.unwrap();
    let playoffs = get_player_stats(&client, "josh-allen", None, &Season::Playoffs).await.unwrap();
    
    assert_eq!(regular.season, "regular");
    assert_eq!(playoffs.season, "playoffs");
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
        primary_position: "QB".to_string(),
        jersey_number: 1,
        current_team: "CUSTOM".to_string(),
        injury: String::new(),
        rookie: false,
        games_played: 16,
        season: "2024-regular".to_string(),
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
    let stats = get_player_stats(&client, "josh-allen", None, &Season::Regular).await.unwrap();
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
use statbook::{StatsProvider, NewsProvider, PlayerStats, PlayerNews, Article, NewsQuery, Result, StatbookClient};
use async_trait::async_trait;
use std::sync::Arc;

struct MyCustomStatsProvider;

#[async_trait]
impl StatsProvider for MyCustomStatsProvider {
    async fn fetch_player_stats(&self, name: &str, season: &str) -> Result<PlayerStats> {
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
            season: season.to_string(),
        })
    }
}

struct MyCustomNewsProvider;

#[async_trait]
impl NewsProvider for MyCustomNewsProvider {
    async fn fetch_player_news(&self, query: &NewsQuery) -> Result<PlayerNews> {
        // Your custom news implementation
        let articles = vec![Article {
            title: format!("Custom news about {}", query.player_name),
            description: "Custom news description".to_string(),
            published_at: "2024-01-01T00:00:00Z".to_string(),
            content: "Custom news content".to_string(),
        }];
        Ok(PlayerNews::new(articles, query.clone()))
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
