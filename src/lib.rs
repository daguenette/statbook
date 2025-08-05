//! # Statbook
//!
//! A Rust library for accessing sports statistics and data.
//!
//! Currently supports NFL player data via [MySportsFeeds.com](https://www.mysportsfeeds.com)
//! with plans to expand to other sports and data sources.
//!
//! ## Quick Start
//!
//! First, set up your MySportsFeeds credentials:
//!
//! ```bash
//! export STATBOOK_API_KEY="your-api-key"
//! export STATBOOK_PASSWORD="your-password"
//! ```
//!
//! Then use the library:
//!
//! ```rust,no_run
//! use statbook::{StatbookClient, api::players::get_player_by_name};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = StatbookClient::from_env()?;
//!     
//!     if let Some(player) = get_player_by_name(&client, "tom-brady").await? {
//!         println!("{} {} - #{}", player.first_name, player.last_name, player.jersey_number);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration Options
//!
//! ```rust,no_run
//! use statbook::{StatbookClient, config::StatbookConfig};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // From environment variables
//! let client = StatbookClient::from_env()?;
//!
//! // Using builder pattern
//! let config = StatbookConfig::builder()
//!     .api_key("your-key")
//!     .password("your-password")
//!     .build()?;
//! let client = StatbookClient::new(config);
//!
//! // Direct configuration
//! let config = StatbookConfig::new("your-key".to_string(), "your-password".to_string());
//! let client = StatbookClient::new(config);
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod client;
pub mod config;
pub mod error;
pub mod models;
mod utils;

// Re-export main types for convenience
pub use client::StatbookClient;
pub use config::StatbookConfig;
pub use error::{Result, StatbookError};
pub use models::PlayerSummary;
