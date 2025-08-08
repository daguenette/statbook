use crate::error::{Result, StatbookError};
use std::env;

pub mod news_config;
pub use news_config::{NewsConfig, SortBy};

const STATS_BASE_URL: &str = "https://api.mysportsfeeds.com/v2.1";
const NEWS_BASE_URL: &str = "https://newsapi.org/v2";

/// Configuration for the Statbook client.
///
/// This struct contains all the necessary configuration for connecting to
/// sports statistics and news APIs, including API keys, base URLs, and
/// news-specific settings.
#[derive(Debug, Clone)]
pub struct StatbookConfig {
    /// API key for the statistics provider
    pub stats_api_key: String,
    /// API key for the news provider
    pub news_api_key: String,
    /// Base URL for the statistics API
    pub stats_base_url: String,
    /// Base URL for the news API
    pub news_base_url: String,
    /// Configuration specific to news fetching
    pub news_config: NewsConfig,
}

impl StatbookConfig {
    /// Creates a new configuration with the provided API keys.
    ///
    /// This constructor uses default base URLs and news configuration.
    /// For more control over the configuration, use the builder pattern.
    ///
    /// # Arguments
    ///
    /// * `stats_api_key` - API key for the statistics provider
    /// * `news_api_key` - API key for the news provider
    pub fn new(stats_api_key: String, news_api_key: String) -> Self {
        Self {
            stats_api_key,
            news_api_key,
            stats_base_url: STATS_BASE_URL.to_string(),
            news_base_url: NEWS_BASE_URL.to_string(),
            news_config: NewsConfig::default(),
        }
    }

    /// Creates configuration from environment variables.
    ///
    /// This method reads configuration from the following environment variables:
    /// - `STATS_API_KEY` - Required, API key for statistics provider
    /// - `NEWS_API_KEY` - Required, API key for news provider
    /// - `STATS_BASE_URL` - Optional, defaults to MySports API
    /// - `NEWS_BASE_URL` - Optional, defaults to NewsAPI
    ///
    /// # Returns
    ///
    /// Returns `Ok(StatbookConfig)` if all required environment variables are
    /// present and valid, otherwise returns an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Required environment variables are missing
    /// - API keys are empty
    /// - URLs are not valid HTTP URLs
    pub fn from_env() -> Result<Self> {
        let stats_api_key =
            env::var("STATS_API_KEY").map_err(|_| StatbookError::MissingApiKey {
                key: "STATS_API_KEY".to_string(),
            })?;

        let news_api_key = env::var("NEWS_API_KEY").map_err(|_| StatbookError::MissingApiKey {
            key: "NEWS_API_KEY".to_string(),
        })?;

        let config = Self {
            stats_api_key,
            news_api_key,
            stats_base_url: STATS_BASE_URL.to_string(),
            news_base_url: NEWS_BASE_URL.to_string(),
            news_config: NewsConfig::default(),
        };

        config.validate()?;
        Ok(config)
    }

    /// Validates the configuration.
    ///
    /// This method checks that all required fields are present and valid,
    /// including API keys and URLs.
    ///
    /// # Errors
    ///
    /// Returns an error if any validation checks fail.
    pub fn validate(&self) -> Result<()> {
        self.validate_api_keys()?;
        self.validate_urls()?;
        Ok(())
    }

    /// Validates that API keys are not empty.
    ///
    /// The actual format validation is left to the respective APIs,
    /// which can provide more specific error messages.
    fn validate_api_keys(&self) -> Result<()> {
        if self.stats_api_key.is_empty() {
            return Err(StatbookError::Validation(
                "Stats API key cannot be empty".to_string(),
            ));
        }
        if self.news_api_key.is_empty() {
            return Err(StatbookError::Validation(
                "News API key cannot be empty".to_string(),
            ));
        }

        // Let the APIs themselves validate key formats
        // This provides consistent validation and better error messages from the source
        Ok(())
    }

    /// Validates that URLs are properly formatted HTTP URLs.
    fn validate_urls(&self) -> Result<()> {
        if !self.stats_base_url.starts_with("http") {
            return Err(StatbookError::Validation(
                "Stats base URL must be a valid HTTP URL".to_string(),
            ));
        }
        if !self.news_base_url.starts_with("http") {
            return Err(StatbookError::Validation(
                "News base URL must be a valid HTTP URL".to_string(),
            ));
        }
        Ok(())
    }

    /// Returns a new configuration with the specified news configuration.
    ///
    /// # Arguments
    ///
    /// * `news_config` - The news configuration to use
    pub fn with_news_config(mut self, news_config: NewsConfig) -> Self {
        self.news_config = news_config;
        self
    }

    /// Returns a new configuration builder.
    ///
    /// This is the recommended way to create a `StatbookConfig` when you need
    /// to customize multiple settings.
    pub fn builder() -> StatbookConfigBuilder {
        StatbookConfigBuilder::default()
    }
}

/// Builder for creating `StatbookConfig` instances.
///
/// This builder provides a fluent interface for constructing configuration
/// with validation. It ensures all required fields are provided before
/// creating the final configuration.
#[derive(Default)]
pub struct StatbookConfigBuilder {
    stats_api_key: Option<String>,
    news_api_key: Option<String>,
    stats_base_url: Option<String>,
    news_base_url: Option<String>,
    news_config: Option<NewsConfig>,
}

impl StatbookConfigBuilder {
    /// Sets the statistics API key.
    ///
    /// # Arguments
    ///
    /// * `stats_api_key` - The API key for the statistics provider
    pub fn stats_api_key<S: Into<String>>(mut self, stats_api_key: S) -> Self {
        self.stats_api_key = Some(stats_api_key.into());
        self
    }

    /// Sets the news API key.
    ///
    /// # Arguments
    ///
    /// * `news_api_key` - The API key for the news provider
    pub fn news_api_key<S: Into<String>>(mut self, news_api_key: S) -> Self {
        self.news_api_key = Some(news_api_key.into());
        self
    }

    /// Sets a custom base URL for the statistics API.
    ///
    /// If not provided, defaults to the MySports API URL.
    ///
    /// # Arguments
    ///
    /// * `stats_base_url` - The base URL for the statistics API
    pub fn stats_base_url<S: Into<String>>(mut self, stats_base_url: S) -> Self {
        self.stats_base_url = Some(stats_base_url.into());
        self
    }

    /// Sets a custom base URL for the news API.
    ///
    /// If not provided, defaults to the NewsAPI URL.
    ///
    /// # Arguments
    ///
    /// * `news_base_url` - The base URL for the news API
    pub fn news_base_url<S: Into<String>>(mut self, news_base_url: S) -> Self {
        self.news_base_url = Some(news_base_url.into());
        self
    }

    /// Sets the news configuration.
    ///
    /// If not provided, uses default news configuration settings.
    ///
    /// # Arguments
    ///
    /// * `news_config` - Configuration for news fetching behavior
    pub fn news_config(mut self, news_config: NewsConfig) -> Self {
        self.news_config = Some(news_config);
        self
    }

    /// Builds the final configuration.
    ///
    /// This method validates that all required fields are present and
    /// creates a `StatbookConfig` instance.
    ///
    /// # Returns
    ///
    /// Returns `Ok(StatbookConfig)` if the configuration is valid,
    /// otherwise returns an error describing what's missing or invalid.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Required API keys are missing
    /// - API keys are empty
    /// - URLs are not valid HTTP URLs
    pub fn build(self) -> Result<StatbookConfig> {
        let stats_api_key = self.stats_api_key.ok_or(StatbookError::MissingApiKey {
            key: "stats_api_key".to_string(),
        })?;
        let news_api_key = self.news_api_key.ok_or(StatbookError::MissingApiKey {
            key: "news_api_key".to_string(),
        })?;
        let stats_base_url = self
            .stats_base_url
            .unwrap_or_else(|| STATS_BASE_URL.to_string());
        let news_base_url = self
            .news_base_url
            .unwrap_or_else(|| NEWS_BASE_URL.to_string());
        let news_config = self.news_config.unwrap_or_default();

        let config = StatbookConfig {
            stats_api_key,
            news_api_key,
            stats_base_url,
            news_base_url,
            news_config,
        };

        config.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_consistent_validation() {
        // Empty stats key should fail
        let result = StatbookConfig::builder()
            .stats_api_key("")
            .news_api_key("any-key")
            .build();
        assert!(result.is_err());

        // Empty news key should fail
        let result = StatbookConfig::builder()
            .stats_api_key("any-key")
            .news_api_key("")
            .build();
        assert!(result.is_err());

        // Short keys should now pass validation (consistent behavior)
        let result = StatbookConfig::builder()
            .stats_api_key("short")
            .news_api_key("also-short")
            .build();
        assert!(
            result.is_ok(),
            "Short keys should pass validation - APIs will validate format"
        );

        // Different length keys should both pass (consistent behavior)
        let result = StatbookConfig::builder()
            .stats_api_key("a-20-character-key-x")
            .news_api_key("different-length-key")
            .build();
        assert!(
            result.is_ok(),
            "Different length keys should both pass validation"
        );
    }
}
