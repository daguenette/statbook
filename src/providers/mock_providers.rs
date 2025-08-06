use crate::{
    error::{Result, StatbookError},
    models::{Article, NewsQuery, PlayerStats},
    providers::{NewsProvider, StatsProvider},
};
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Debug)]
pub struct MockStatsProvider {
    responses: HashMap<String, PlayerStats>,
    errors: HashMap<String, StatbookError>,
}

impl MockStatsProvider {
    pub fn new() -> Self {
        Self {
            responses: HashMap::new(),
            errors: HashMap::new(),
        }
    }

    pub fn with_defaults() -> Self {
        let mut provider = Self::new();

        provider.add_player_stats(
            "josh-allen",
            PlayerStats {
                first_name: "Josh".to_string(),
                last_name: "Allen".to_string(),
                primary_position: "QB".to_string(),
                jersey_number: 17,
                current_team: "BUF".to_string(),
                injury: "".to_string(),
                rookie: false,
                games_played: 16,
            },
        );

        provider.add_player_stats(
            "tom-brady",
            PlayerStats {
                first_name: "Tom".to_string(),
                last_name: "Brady".to_string(),
                primary_position: "QB".to_string(),
                jersey_number: 12,
                current_team: "TB".to_string(),
                injury: "".to_string(),
                rookie: false,
                games_played: 17,
            },
        );

        provider
    }

    pub fn add_player_stats(&mut self, name: &str, stats: PlayerStats) {
        self.responses.insert(name.to_string(), stats);
    }

    pub fn add_player_error(&mut self, name: &str, error: StatbookError) {
        self.errors.insert(name.to_string(), error);
    }

    pub fn add_player_not_found(&mut self, name: &str) {
        self.errors.insert(
            name.to_string(),
            StatbookError::PlayerNotFound {
                name: name.to_string(),
            },
        );
    }
}

#[async_trait]
impl StatsProvider for MockStatsProvider {
    async fn fetch_player_stats(&self, name: &str) -> Result<PlayerStats> {
        if let Some(_error) = self.errors.get(name) {
            return Err(StatbookError::PlayerNotFound {
                name: name.to_string(),
            });
        }

        match self.responses.get(name) {
            Some(stats) => Ok(stats.clone()),
            None => Err(StatbookError::PlayerNotFound {
                name: name.to_string(),
            }),
        }
    }
}

#[derive(Debug)]
pub struct MockNewsProvider {
    responses: HashMap<String, Vec<Article>>,
    errors: HashMap<String, StatbookError>,
}

impl MockNewsProvider {
    pub fn new() -> Self {
        Self {
            responses: HashMap::new(),
            errors: HashMap::new(),
        }
    }

    pub fn with_defaults() -> Self {
        let mut provider = Self::new();

        // Add some default test data
        provider.add_news_articles(
            "josh-allen",
            vec![
                Article {
                    title: "Josh Allen leads Bills to victory".to_string(),
                    description: "Quarterback throws for 300 yards".to_string(),
                    published_at: "2024-01-15T10:00:00Z".to_string(),
                    content: "Full article content here...".to_string(),
                },
                Article {
                    title: "Allen named AFC Player of the Week".to_string(),
                    description: "Recognition for outstanding performance".to_string(),
                    published_at: "2024-01-14T15:30:00Z".to_string(),
                    content: "More article content...".to_string(),
                },
            ],
        );

        provider.add_news_articles(
            "tom-brady",
            vec![Article {
                title: "Brady announces retirement".to_string(),
                description: "Legendary quarterback calls it a career".to_string(),
                published_at: "2024-01-10T12:00:00Z".to_string(),
                content: "Retirement announcement content...".to_string(),
            }],
        );

        provider
    }

    pub fn add_news_articles(&mut self, player_name: &str, articles: Vec<Article>) {
        self.responses.insert(player_name.to_string(), articles);
    }

    pub fn add_news_error(&mut self, player_name: &str, error: StatbookError) {
        self.errors.insert(player_name.to_string(), error);
    }
}

#[async_trait]
impl NewsProvider for MockNewsProvider {
    async fn fetch_player_news(&self, query: &NewsQuery) -> Result<Vec<Article>> {
        if let Some(_error) = self.errors.get(&query.player_name) {
            return Err(StatbookError::NewsApi {
                status: 500,
                message: "Mock news error".to_string(),
            });
        }

        match self.responses.get(&query.player_name) {
            Some(articles) => Ok(articles.clone()),
            None => Ok(vec![]), // Return empty vec for unknown players
        }
    }
}

impl Default for MockStatsProvider {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl Default for MockNewsProvider {
    fn default() -> Self {
        Self::with_defaults()
    }
}
