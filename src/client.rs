use crate::{config::StatbookConfig, error::Result, models::response::PlayerResponse};
use base64::{engine::general_purpose, Engine as _};

#[derive(Debug, Clone)]
pub struct StatbookClient {
    config: StatbookConfig,
    http_client: reqwest::Client,
}

impl StatbookClient {
    pub fn new(config: StatbookConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
        }
    }

    pub fn from_env() -> Result<Self> {
        let config = StatbookConfig::from_env()?;
        Ok(Self::new(config))
    }

    pub fn builder() -> crate::config::StatbookConfigBuilder {
        StatbookConfig::builder()
    }

    pub(crate) async fn fetch_raw_player_data(&self, name: &str) -> Result<PlayerResponse> {
        let credentials = format!("{}:{}", self.config.api_key, self.config.password);
        let encoded_credentials = general_purpose::STANDARD.encode(&credentials);
        let auth_header = format!("Basic {encoded_credentials}");
        let url = format!(
            "{}/pull/nfl/latest/player_stats_totals.json",
            self.config.base_url
        );

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", auth_header)
            .query(&[("player", name)])
            .send()
            .await?;

        let json = response.text().await?;
        let player_data: PlayerResponse = serde_json::from_str(&json)?;

        Ok(player_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::StatbookConfig;

    #[tokio::test]
    async fn test_fetch_raw_player_data() {
        dotenv::dotenv().ok();

        let config = match StatbookConfig::from_env() {
            Ok(config) => config,
            Err(_) => {
                eprintln!("Skipping test - no credentials provided");
                return;
            }
        };

        let client = StatbookClient::new(config);
        let result = client.fetch_raw_player_data("josh-allen").await;

        match result {
            Ok(player_data) => {
                eprintln!("Found {} players", player_data.players.len());
            }
            Err(e) => {
                eprintln!("Error: {e}")
            }
        }
    }
}
