/// Complete player information including statistics and news articles.
///
/// This struct combines player statistics with related news articles,
/// providing a comprehensive view of a player's current status and
/// recent coverage.
///
/// # Examples
///
/// ```rust
/// use statbook::{StatbookClient, FetchStrategy, api::players::get_player_summary};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = StatbookClient::from_env()?;
/// let result = get_player_summary(&client, "LeBron James", FetchStrategy::Both {
///     fail_on_news_error: false
/// }).await?;
///
/// let stats = &result.player_stats;
/// println!("{} {} plays {} for the {}",
///     stats.first_name,
///     stats.last_name,
///     stats.primary_position,
///     stats.current_team
/// );
///
/// if let Ok(articles) = &result.news_result {
///     for article in articles {
///         println!("News: {}", article.title);
///     }
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PlayerSummary {
    /// Player's first name
    pub first_name: String,
    /// Player's last name
    pub last_name: String,
    /// Primary playing position (e.g., "Point Guard", "Quarterback")
    pub primary_position: String,
    /// Jersey number
    pub jersey_number: u32,
    /// Current team name
    pub current_team: String,
    /// Current injury status (empty string if healthy)
    pub injury: String,
    /// Whether this is the player's rookie season
    pub rookie: bool,
    /// Total games played this season
    pub games_played: u64,
    /// Recent news articles about the player
    pub news_articles: Vec<Article>,
}

/// A news article about a player or team.
///
/// This struct represents a single news article with metadata
/// about publication time and content.
///
/// # Examples
///
/// ```rust
/// use statbook::Article;
///
/// fn print_article(article: &Article) {
///     println!("Title: {}", article.title);
///     println!("Published: {}", article.published_at);
///     println!("Summary: {}", article.description);
/// }
/// ```
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
