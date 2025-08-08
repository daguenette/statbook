/// Strategy for fetching player data.
///
/// This enum is preserved for future use in functions like `get_player_report()`
/// where different strategies might control what sections of a report to include.
/// Currently not used by `get_player_summary()` which always fetches both stats and news.

#[allow(dead_code)]
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

/// Season Selector for fetching player data.
#[derive(Debug, Clone)]
pub enum Season {
    Current,
    Latest,
    Upcoming,
    Regular,
    Playoffs,
}

impl Season {
    pub fn as_str(&self) -> &'static str {
        match self {
            Season::Current => "current",
            Season::Latest => "latest",
            Season::Upcoming => "upcoming",
            Season::Regular => "regular",
            Season::Playoffs => "playoff",
        }
    }

    pub fn format_with_years(&self, start_year: i64, end_year: i64) -> String {
        format!("{start_year}-{end_year}-{}", self.as_str())
    }
}
