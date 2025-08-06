use crate::{
    config::StatbookConfig,
    error::{Result, StatbookError},
    models::{player_parser::PlayerResponse, PlayerStats},
};
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};

const STATBOOK_PASSWORD: &str = "MYSPORTSFEEDS";

/// Trait for providing player statistics from various data sources.
///
/// This trait abstracts the statistics fetching logic, allowing for
/// different implementations (real APIs, mock data, cached data, etc.).
/// The trait is designed to be thread-safe and async-compatible.
///
/// # Examples
///
/// ```rust
/// use statbook::{StatsProvider, PlayerStats, Result};
/// use async_trait::async_trait;
///
/// struct CustomStatsProvider;
///
/// #[async_trait]
/// impl StatsProvider for CustomStatsProvider {
///     async fn fetch_player_stats(&self, name: &str) -> Result<PlayerStats> {
///         // Custom implementation
///         todo!()
///     }
/// }
/// ```
#[async_trait]
pub trait StatsProvider: Send + Sync {
    /// Fetches player statistics by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The player name to search for (e.g., "josh-allen", "LeBron James")
    ///
    /// # Returns
    ///
    /// Returns `Ok(PlayerStats)` if the player is found and data is successfully
    /// retrieved, otherwise returns an error.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The player is not found
    /// - Network request fails
    /// - API returns an error response
    /// - Response parsing fails
    async fn fetch_player_stats(&self, name: &str) -> Result<PlayerStats>;
}

/// MySports API implementation of the `StatsProvider` trait.
///
/// This provider fetches player statistics from the MySports API,
/// which provides comprehensive NFL player data including current
/// season statistics, team information, and injury status.
///
/// # Authentication
///
/// MySports API uses HTTP Basic Authentication with the API key as
/// username and a fixed password.
///
/// # Examples
///
/// ```rust
/// use statbook::{StatbookClient, api::players::get_player_stats};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = StatbookClient::from_env()?;
/// let stats = get_player_stats(&client, "josh-allen").await?;
///
/// println!("{} plays for {}", stats.first_name, stats.current_team);
/// # Ok(())
/// # }
/// ```
pub struct MySportsStatsProvider {
    config: StatbookConfig,
    http_client: reqwest::Client,
}

impl MySportsStatsProvider {
    /// Creates a new MySports statistics provider.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration containing the MySports API key and base URL
    ///
    /// # Examples
    ///
    /// ```rust
    /// use statbook::{StatbookClient, StatbookConfig};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = StatbookConfig::builder()
    ///     .stats_api_key("your-api-key")
    ///     .news_api_key("your-news-key")
    ///     .build()?;
    ///
    /// let client = StatbookClient::new(config);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(config: StatbookConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl StatsProvider for MySportsStatsProvider {
    async fn fetch_player_stats(&self, name: &str) -> Result<PlayerStats> {
        let credentials = format!("{}:{}", self.config.stats_api_key, STATBOOK_PASSWORD);
        let encoded_credentials = general_purpose::STANDARD.encode(&credentials);
        let auth_header = format!("Basic {encoded_credentials}");
        let url = format!(
            "{}/pull/nfl/latest/player_stats_totals.json",
            self.config.stats_base_url
        );

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", auth_header)
            .query(&[("player", name)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(StatbookError::StatsApi {
                status: response.status().as_u16(),
                message: format!("Failed to fetch player stats for '{name}'"),
            });
        }

        let json = response.text().await?;
        let player_data: PlayerResponse = serde_json::from_str(&json)?;

        let player = match player_data.players.first() {
            Some(player) => player,
            None => {
                return Err(StatbookError::PlayerNotFound {
                    name: name.to_string(),
                })
            }
        };

        let player_info = &player.player_info;

        Ok(PlayerStats {
            first_name: player_info.first_name.clone().unwrap_or_default(),
            last_name: player_info.last_name.clone().unwrap_or_default(),
            primary_position: player_info.primary_position.clone().unwrap_or_default(),
            jersey_number: player_info.jersey_number.unwrap_or(0),
            current_team: player_info
                .current_team
                .as_ref()
                .and_then(|team| team.abbreviation.clone())
                .unwrap_or_default(),
            injury: player_info.injury.clone().unwrap_or_default(),
            rookie: player_info.rookie.unwrap_or(false),
            games_played: player.statistics.games_played.unwrap_or(0),
        })
    }
}
