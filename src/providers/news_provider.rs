use crate::{
    config::StatbookConfig,
    error::{Result, StatbookError},
    models::{parsers::news_parser::NewsResponse, Article, NewsQuery},
};
use async_trait::async_trait;

/// Trait for providing news articles from various data sources.
///
/// This trait abstracts the news fetching logic, allowing for different
/// implementations (real APIs, mock data, cached data, etc.). The trait
/// is designed to be thread-safe and async-compatible.
#[async_trait]
pub trait NewsProvider: Send + Sync {
    /// Fetches news articles based on the provided query.
    ///
    /// # Arguments
    ///
    /// * `query` - Search parameters including player name, date range, and limits
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<Article>)` containing matching news articles,
    /// or an error if the request fails.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - Network request fails
    /// - API returns an error response (e.g., invalid API key, rate limit)
    /// - Response parsing fails
    async fn fetch_player_news(&self, query: &NewsQuery) -> Result<Vec<Article>>;
}

/// NewsAPI implementation of the `NewsProvider` trait.
///
/// This provider fetches news articles from NewsAPI.org, which aggregates
/// news from thousands of sources worldwide. It's particularly useful for
/// getting recent coverage of sports players and teams.
///
/// # Free Tier Compatibility
///
/// This implementation is compatible with NewsAPI's free tier by default.
/// Date filtering (using the `from` parameter) is only applied when explicitly
/// set via `NewsQuery::with_date_range()`, as it requires a paid subscription.
///
/// # Rate Limits
///
/// NewsAPI has rate limits that vary by subscription tier. The free tier
/// allows 1,000 requests per day.
pub struct NewsApiProvider {
    config: StatbookConfig,
    http_client: reqwest::Client,
}

impl NewsApiProvider {
    /// Creates a new NewsAPI provider.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration containing the NewsAPI key and base URL
    pub fn new(config: StatbookConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl NewsProvider for NewsApiProvider {
    async fn fetch_player_news(&self, query: &NewsQuery) -> Result<Vec<Article>> {
        let url = format!("{}/everything", self.config.news_base_url);

        // Build query parameters, only including 'from' if it's not empty
        // This ensures compatibility with NewsAPI free tier which doesn't support date filtering
        let page_size_str = query.page_size.to_string();
        let mut query_params = vec![
            ("q", query.player_name.as_str()),
            ("pageSize", page_size_str.as_str()),
            ("sortBy", query.sort_by.as_str()),
            ("apiKey", self.config.news_api_key.as_str()),
        ];

        // Only add 'from' parameter if it's not empty (paid tier feature)
        if !query.from_date.is_empty() {
            query_params.push(("from", query.from_date.as_str()));
        }

        let response = self
            .http_client
            .get(&url)
            .header("User-Agent", "FantasyFootballApp/1.0")
            .query(&query_params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(StatbookError::NewsApi {
                status: response.status().as_u16(),
                message: format!("Failed to fetch news for '{}'", query.player_name),
            });
        }

        let json = response.text().await?;
        let news_data: NewsResponse = serde_json::from_str(&json)?;

        let articles = news_data
            .articles
            .iter()
            .map(|article| Article {
                title: article.title.clone().unwrap_or_default(),
                content: article.content.clone().unwrap_or_default(),
                description: article.description.clone().unwrap_or_default(),
                published_at: article.published_at.clone().unwrap_or_default(),
            })
            .collect();

        Ok(articles)
    }
}
