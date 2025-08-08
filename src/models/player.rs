use crate::models::news::Article;

/// Player statistics without news articles.
///
/// This struct contains core player information and statistics,
/// used when only statistical data is needed or as part of
/// partial fetch results.

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
    /// Season for which these stats apply
    pub season: String,
}

/// Quick summary of essential player information with news.
///
/// This struct contains only the most important player details
/// for summary/overview purposes. Use `get_player_stats()` for
/// complete statistical information including injury status and rookie info.

#[derive(Debug, Clone)]
pub struct PlayerSummary {
    /// Player's first name
    pub first_name: String,
    /// Player's last name
    pub last_name: String,
    /// Primary playing position
    pub primary_position: String,
    /// Current team name
    pub current_team: String,
    /// Jersey number
    pub jersey_number: u32,
    /// Total games played this season
    pub games_played: u64,
    /// Related news articles
    pub news: Vec<Article>,
}