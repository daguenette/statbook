use serde::{Deserialize, Serialize};

/// A news article about a player or team.
///
/// This struct represents a single news article with metadata
/// about publication time and content.

#[derive(Debug, Clone)]
pub struct Article {
    /// Article headline
    pub title: String,
    /// Brief description or summary of the article
    pub description: String,
    /// Publication timestamp (ISO 8601 format)
    pub published_at: String,
    /// Full article content (may be truncated)
    pub content: String,
}

/// Query parameters for fetching news articles.
///
/// This struct encapsulates the parameters used to search for news
/// articles, including player name, date range, and result limits.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsQuery {
    /// Name of the player to search for
    pub player_name: String,
    /// Start date for news search (YYYY-MM-DD format)
    pub from_date: String,
    /// Maximum number of articles to return
    pub page_size: u32,
    /// Field to sort results by (e.g., "publishedAt")
    pub sort_by: String,
}

impl NewsQuery {
    /// Creates a news query for a player with default parameters.
    ///
    /// The default query returns up to 5 articles sorted by publication date.
    /// No date filtering is applied to ensure compatibility with NewsAPI free tier.
    /// Use `with_date_range()` to add date filtering if you have a paid NewsAPI plan.
    ///
    /// # Arguments
    ///
    /// * `name` - The player name to search for
    pub fn for_player(name: &str) -> Self {
        Self {
            player_name: name.to_string(),
            from_date: String::new(), // Empty string = no date filter for free tier compatibility
            page_size: 5,
            sort_by: "publishedAt".to_string(),
        }
    }

    /// Sets the maximum number of articles to return.
    ///
    /// # Arguments
    ///
    /// * `size` - Maximum number of articles (typically 1-100)
    pub fn with_page_size(mut self, size: u32) -> Self {
        self.page_size = size;
        self
    }

    /// Sets the start date for the news search.
    ///
    /// **Note:** Date filtering requires a paid NewsAPI subscription.
    /// The free tier will return a 426 error if this parameter is used.
    ///
    /// # Arguments
    ///
    /// * `from_date` - Start date in YYYY-MM-DD format
    pub fn with_date_range(mut self, from_date: String) -> Self {
        self.from_date = from_date;
        self
    }
}
/// Collection of news articles for a player.
///
/// This struct wraps a collection of news articles with metadata
/// about the query used to fetch them, providing a more structured
/// and extensible way to return news data.

#[derive(Debug, Clone)]
pub struct PlayerNews {
    /// Collection of news articles
    pub articles: Vec<Article>,
    /// Query used to fetch these articles (for debugging/logging)
    pub query: NewsQuery,
    /// Total articles available (if API provides this)
    pub total_count: Option<u32>,
}

impl PlayerNews {
    /// Creates a new PlayerNews instance.
    ///
    /// # Arguments
    ///
    /// * `articles` - Vector of news articles
    /// * `query` - The query used to fetch these articles
    pub fn new(articles: Vec<Article>, query: NewsQuery) -> Self {
        Self {
            articles,
            query,
            total_count: None,
        }
    }

    /// Sets the total count of articles available.
    ///
    /// # Arguments
    ///
    /// * `count` - Total number of articles available from the API
    pub fn with_total_count(mut self, count: u32) -> Self {
        self.total_count = Some(count);
        self
    }

    /// Returns the number of articles in this collection.
    pub fn len(&self) -> usize {
        self.articles.len()
    }

    /// Returns true if there are no articles in this collection.
    pub fn is_empty(&self) -> bool {
        self.articles.is_empty()
    }
}
