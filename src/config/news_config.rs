use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsConfig {
    pub max_articles: u32,
    pub days_back: u32,
    pub sort_by: SortBy,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortBy {
    #[serde(rename = "publishedAt")]
    PublishedAt,
    #[serde(rename = "relevancy")]
    Relevancy,
    #[serde(rename = "popularity")]
    Popularity,
}

impl Default for NewsConfig {
    fn default() -> Self {
        Self {
            max_articles: 5,
            days_back: 7,
            sort_by: SortBy::PublishedAt,
            language: "en".to_string(),
        }
    }
}

impl NewsConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_articles(mut self, max: u32) -> Self {
        self.max_articles = max;
        self
    }

    pub fn with_days_back(mut self, days: u32) -> Self {
        self.days_back = days;
        self
    }

    pub fn with_sort_by(mut self, sort_by: SortBy) -> Self {
        self.sort_by = sort_by;
        self
    }

    pub fn with_language(mut self, language: String) -> Self {
        self.language = language;
        self
    }
}

impl std::fmt::Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortBy::PublishedAt => write!(f, "publishedAt"),
            SortBy::Relevancy => write!(f, "relevancy"),
            SortBy::Popularity => write!(f, "popularity"),
        }
    }
}
