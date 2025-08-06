//! # Statbook
//!
//! A high-performance Rust library for accessing sports statistics and news data with concurrent API calls,
//! comprehensive error handling, and flexible configuration options.
//!
//! Currently supports NFL player data via [MySportsFeeds.com](https://www.mysportsfeeds.com)
//! and news data via [NewsAPI.org](https://newsapi.org) with plans to expand to other sports and data sources.
//!
//! ## Features
//!
//! - **Concurrent API calls** for improved performance
//! - **Comprehensive error handling** with detailed error types
//! - **Flexible configuration** with builder pattern and environment variables
//! - **Built-in testing utilities** with mock providers
//! - **Multiple fetch strategies** (stats-only, news-only, or both)
//! - **Extensible architecture** with trait-based providers
//!
//! ## How-to
//!
//! ### Set up API credentials:
//!
//! ```bash
//! export STATS_API_KEY="your-mysportsfeeds-api-key"
//! export NEWS_API_KEY="your-newsapi-key"
//! ```
//! ### Flexible API Functions
//!
//! ```rust,no_run
//! use statbook::{
//!     StatbookClient, FetchStrategy, NewsQuery,
//!     api::players::{get_player_stats, get_player_news, get_player_summary}
//! };
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = StatbookClient::from_env()?;
//!
//! // Get only player statistics (faster)
//! let stats = get_player_stats(&client, "josh-allen").await?;
//!
//! // Get only news articles
//! let query = NewsQuery::for_player("josh-allen").with_page_size(10);
//! let news = get_player_news(&client, &query).await?;
//!
//! // Get both with concurrent fetching and partial failure handling
//! let result = get_player_summary(&client, "josh-allen",
//!     FetchStrategy::Both { fail_on_news_error: false }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Custom Configuration
//!
//! ```rust,no_run
//! use statbook::{StatbookClient, StatbookConfig, NewsConfig, SortBy};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Custom news configuration
//! let news_config = NewsConfig::new()
//!     .with_max_articles(15)
//!     .with_days_back(30)
//!     .with_sort_by(SortBy::Relevancy);
//!
//! // Build configuration
//! let config = StatbookConfig::builder()
//!     .stats_api_key("your-stats-key")
//!     .news_api_key("your-news-key")
//!     .news_config(news_config)
//!     .build()?;
//!
//! let client = StatbookClient::new(config);
//! # Ok(())
//! # }
//! ```
//!
//! ### Error Handling
//!
//! ```rust,no_run
//! use statbook::{StatbookClient, StatbookError, api::players::get_player_stats};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = StatbookClient::from_env()?;
//!
//! match get_player_stats(&client, "unknown-player").await {
//!     Ok(stats) => println!("Found: {} {}", stats.first_name, stats.last_name),
//!     Err(StatbookError::PlayerNotFound { name }) => {
//!         println!("No player named '{}'", name);
//!     }
//!     Err(StatbookError::Network(e)) => {
//!         println!("Network error: {}", e);
//!     }
//!     Err(e) => println!("Other error: {}", e),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Testing
//!
//! The library provides built-in testing utilities:
//!
//! ```rust
//! use statbook::{create_mock_client, api::players::get_player_stats};
//!
//! #[tokio::test]
//! async fn test_my_app() {
//!     let client = create_mock_client();  // No real API calls
//!     let stats = get_player_stats(&client, "josh-allen").await.unwrap();
//!     assert_eq!(stats.first_name, "Josh");
//! }
//! ```

pub mod api;
pub mod client;
pub mod config;
pub mod error;

mod models;
mod providers;
mod test_utils;
mod utils;

// Re-export main types for convenience
pub use client::StatbookClient;
pub use config::{NewsConfig, SortBy, StatbookConfig};
pub use error::{Result, StatbookError};
pub use models::{
    Article, FetchStrategy, NewsQuery, PlayerStats, PlayerSummary, PlayerSummaryResult,
};
pub use providers::{MockNewsProvider, MockStatsProvider, NewsProvider, StatsProvider};

// Re-export test utilities directly
pub use test_utils::{
    create_custom_mock_client, create_mock_client, init_integration_test_client,
    skip_if_no_credentials,
};
