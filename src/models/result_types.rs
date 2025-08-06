use crate::{error::StatbookError, models::player::Article};
use serde::{Deserialize, Serialize};

/// Player statistics without news articles.
///
/// This struct contains core player information and statistics,
/// used when only statistical data is needed or as part of
/// partial fetch results.
///
/// # Examples
///
/// ```rust
/// use statbook::{StatbookClient, FetchStrategy, api::players::get_player_summary};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = StatbookClient::from_env()?;
///
/// // Get only stats (faster, no news API call)
/// let result = get_player_summary(&client, "LeBron James", FetchStrategy::StatsOnly).await?;
/// let stats = &result.player_stats;
///
/// println!("{} has played {} games this season",
///     stats.first_name,
///     stats.games_played
/// );
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PlayerStats {
    /// Player's first name
    pub first_name: String,
    /// Player's last name
    pub last_name: String,
    /// Primary playing position
    pub primary_position: String,
    /// Jersey number
    pub jersey_number: u32,
    /// Current team name
    pub current_team: String,
    /// Current injury status (empty if healthy)
    pub injury: String,
    /// Whether this is the player's rookie season
    pub rookie: bool,
    /// Total games played this season
    pub games_played: u64,
}

/// Result of fetching both player stats and news with partial failure support.
///
/// This struct allows for graceful handling of scenarios where player
/// statistics are successfully retrieved but news fetching fails.
///
/// # Examples
///
/// ```rust
/// use statbook::PlayerSummaryResult;
///
/// fn handle_result(result: PlayerSummaryResult) {
///     println!("Player: {} {}",
///         result.player_stats.first_name,
///         result.player_stats.last_name
///     );
///     
///     match result.news_result {
///         Ok(articles) => {
///             println!("Found {} news articles", articles.len());
///         }
///         Err(e) => {
///             println!("News unavailable: {}", e);
///         }
///     }
/// }
/// ```
#[derive(Debug)]
pub struct PlayerSummaryResult {
    /// Player statistics (always present if this struct exists)
    pub player_stats: PlayerStats,
    /// News articles result (may be an error if news fetching failed)
    pub news_result: Result<Vec<Article>, StatbookError>,
}

/// Strategy for fetching player data.
///
/// This enum controls what data to fetch and how to handle failures,
/// allowing for flexible data retrieval based on application needs.
///
/// # Examples
///
/// ```rust
/// use statbook::{StatbookClient, FetchStrategy, api::players::get_player_summary};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = StatbookClient::from_env()?;
///
/// // Fast stats-only fetch
/// let stats_only = get_player_summary(&client, "Player Name", FetchStrategy::StatsOnly).await?;
///
/// // News-only fetch (useful for news aggregation)
/// let news_only = get_player_summary(&client, "Player Name", FetchStrategy::NewsOnly).await?;
///
/// // Both with graceful news failure handling
/// let both_graceful = get_player_summary(&client, "Player Name",
///     FetchStrategy::Both { fail_on_news_error: false }
/// ).await?;
///
/// // Both with strict error handling
/// let both_strict = get_player_summary(&client, "Player Name",
///     FetchStrategy::Both { fail_on_news_error: true }
/// ).await?;
/// # Ok(())
/// # }
/// ```
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

/// Query parameters for fetching news articles.
///
/// This struct encapsulates the parameters used to search for news
/// articles, including player name, date range, and result limits.
///
/// # Examples
///
/// ```rust
/// use statbook::NewsQuery;
///
/// // Simple query for recent news
/// let query = NewsQuery::for_player("LeBron James");
///
/// // Customized query
/// let custom_query = NewsQuery::for_player("LeBron James")
///     .with_page_size(10)
///     .with_date_range("2024-01-01".to_string());
/// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use statbook::NewsQuery;
    ///
    /// let query = NewsQuery::for_player("Stephen Curry");
    /// assert_eq!(query.player_name, "Stephen Curry");
    /// assert_eq!(query.page_size, 5);
    /// assert_eq!(query.from_date, ""); // No date filter by default
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use statbook::NewsQuery;
    ///
    /// let query = NewsQuery::for_player("Player Name")
    ///     .with_page_size(20);
    /// assert_eq!(query.page_size, 20);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use statbook::NewsQuery;
    ///
    /// // Only use with paid NewsAPI subscription
    /// let query = NewsQuery::for_player("Player Name")
    ///     .with_date_range("2024-01-01".to_string());
    /// assert_eq!(query.from_date, "2024-01-01");
    /// ```
    pub fn with_date_range(mut self, from_date: String) -> Self {
        self.from_date = from_date;
        self
    }
}
