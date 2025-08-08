/// Strategy for fetching player data.
///
/// This enum controls what data to fetch and how to handle failures,
/// allowing for flexible data retrieval based on application needs.

#[derive(Debug, Clone)]
pub enum FetchStrategy {
    /// Fetch only player statistics (fastest option)
    StatsOnly,
    /// Fetch only news articles
    NewsOnly,
    /// Fetch both statistics and news
    Both {
        /// Whether to fail the entire operation if news fetching fails
        fail_on_news_error: bool,
    },
}

impl Default for FetchStrategy {
    /// Returns `Both { fail_on_news_error: false }` for graceful degradation.
    fn default() -> Self {
        FetchStrategy::Both {
            fail_on_news_error: false,
        }
    }
}