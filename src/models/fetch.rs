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
