use crate::{error::StatbookError, models::news::Article};

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
}

/// Result of fetching both player stats and news with partial failure support.
///
/// This struct allows for graceful handling of scenarios where player
/// statistics are successfully retrieved but news fetching fails.

#[derive(Debug)]
pub struct PlayerSummaryResult {
    /// Player statistics (always present if this struct exists)
    pub player_stats: PlayerStats,
    /// News articles result (may be an error if news fetching failed)
    pub news_result: Result<Vec<Article>, StatbookError>,
}