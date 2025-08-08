use crate::{
    config::StatbookConfig,
    error::Result,
    providers::{MySportsStatsProvider, NewsApiProvider, NewsProvider, StatsProvider},
};
use std::sync::Arc;

/// The main client for interacting with sports statistics and news APIs.
///
/// `StatbookClient` provides a unified interface for fetching player statistics
/// and news articles from various sports data providers. It uses the provider
/// pattern internally to support different data sources.
pub struct StatbookClient {
    stats_provider: Arc<dyn StatsProvider>,
    news_provider: Arc<dyn NewsProvider>,
}

impl StatbookClient {
    /// Creates a new client with the provided configuration.
    ///
    /// This constructor uses the default providers: `MySportsStatsProvider` for
    /// statistics and `NewsApiProvider` for news articles.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration containing API keys and endpoint URLs
    pub fn new(config: StatbookConfig) -> Self {
        let stats_provider = Arc::new(MySportsStatsProvider::new(config.clone()));
        let news_provider = Arc::new(NewsApiProvider::new(config));

        Self {
            stats_provider,
            news_provider,
        }
    }

    /// Creates a client with custom providers.
    ///
    /// This constructor allows you to inject custom implementations of the
    /// `StatsProvider` and `NewsProvider` traits, useful for testing or
    /// when using alternative data sources.
    ///
    /// # Arguments
    ///
    /// * `stats_provider` - Custom statistics provider implementation
    /// * `news_provider` - Custom news provider implementation
    pub fn with_providers(
        stats_provider: Arc<dyn StatsProvider>,
        news_provider: Arc<dyn NewsProvider>,
    ) -> Self {
        Self {
            stats_provider,
            news_provider,
        }
    }

    /// Creates a client using configuration from environment variables.
    ///
    /// This method reads API keys and configuration from the following
    /// environment variables:
    /// - `STATS_API_KEY` - Required for statistics provider
    /// - `NEWS_API_KEY` - Required for news provider
    /// - `STATS_BASE_URL` - Optional, defaults to MySports API
    /// - `NEWS_BASE_URL` - Optional, defaults to NewsAPI
    ///
    /// # Returns
    ///
    /// Returns `Ok(StatbookClient)` if all required environment variables are set
    /// and valid, otherwise returns an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Required environment variables are missing
    /// - API keys are empty or invalid format
    /// - Configuration validation fails
    pub fn from_env() -> Result<Self> {
        let config = StatbookConfig::from_env()?;
        Ok(Self::new(config))
    }

    /// Returns a configuration builder for creating a client.
    ///
    /// This is a convenience method that returns a `StatbookConfigBuilder`
    /// for fluent configuration setup.
    pub fn builder() -> crate::config::StatbookConfigBuilder {
        StatbookConfig::builder()
    }

    /// Returns a reference to the statistics provider.
    ///
    /// This method provides access to the underlying statistics provider,
    /// useful for advanced use cases or direct provider interaction.
    ///
    /// # Returns
    ///
    /// A reference to the `StatsProvider` trait object.
    pub fn stats_provider(&self) -> &Arc<dyn StatsProvider> {
        &self.stats_provider
    }

    /// Returns a reference to the news provider.
    ///
    /// This method provides access to the underlying news provider,
    /// useful for advanced use cases or direct provider interaction.
    ///
    /// # Returns
    ///
    /// A reference to the `NewsProvider` trait object.
    pub fn news_provider(&self) -> &Arc<dyn NewsProvider> {
        &self.news_provider
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::players::{get_player_news, get_player_stats};
    use crate::models::NewsQuery;
    use crate::test_utils::{create_mock_client, skip_if_no_credentials};

    #[tokio::test]
    async fn test_client_creation_from_env() {
        let _client = skip_if_no_credentials();
    }

    #[tokio::test]
    async fn test_client_with_mock_providers() {
        let client = create_mock_client();

        // Test stats provider
        let stats = get_player_stats(&client, "josh-allen", None, &crate::models::Season::Regular)
            .await
            .unwrap();
        assert_eq!(stats.first_name, "Josh");

        // Test news provider
        let query = NewsQuery::for_player("josh-allen");
        let news = get_player_news(&client, &query).await.unwrap();
        assert!(!news.is_empty());
    }

    #[tokio::test]
    async fn test_client_builder() {
        // Test successful config creation
        let config = StatbookConfig::builder()
            .stats_api_key("test-stats-key")
            .news_api_key("test-news-api-key") // Any non-empty key works
            .build();

        // Should succeed with default URLs
        assert!(config.is_ok());

        // Test validation failure with empty key
        let bad_config = StatbookConfig::builder()
            .stats_api_key("")
            .news_api_key("test-news-api-key")
            .build();

        assert!(bad_config.is_err());
    }
}
