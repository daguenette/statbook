use crate::error::{Result, StatbookError};
use std::env;

#[derive(Debug, Clone)]
pub struct StatbookConfig {
    pub api_key: String,
    pub password: String,
    pub base_url: String,
}

impl StatbookConfig {
    pub fn new(api_key: String, password: String) -> Self {
        Self {
            api_key,
            password,
            base_url: "https://api.mysportsfeeds.com/v2.1".to_string(),
        }
    }

    pub fn from_env() -> Result<Self> {
        let api_key = env::var("STATBOOK_API_KEY").map_err(|_| StatbookError::MissingApiKey)?;

        let password = env::var("STATBOOK_PASSWORD").map_err(|_| StatbookError::MissingPassword)?;

        let base_url = env::var("STATBOOK_BASE_URL")
            .unwrap_or_else(|_| "https://api.mysportsfeeds.com/v2.1".to_string());

        Ok(Self {
            api_key,
            password,
            base_url,
        })
    }

    pub fn builder() -> StatbookConfigBuilder {
        StatbookConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct StatbookConfigBuilder {
    api_key: Option<String>,
    password: Option<String>,
    base_url: Option<String>,
}

impl StatbookConfigBuilder {
    pub fn api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set the password
    pub fn password<S: Into<String>>(mut self, password: S) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Set a custom base URL
    pub fn base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Build the configuration
    pub fn build(self) -> Result<StatbookConfig> {
        let api_key = self.api_key.ok_or(StatbookError::MissingApiKey)?;
        let password = self.password.ok_or(StatbookError::MissingPassword)?;
        let base_url = self
            .base_url
            .unwrap_or_else(|| "https://api.mysportsfeeds.com/v2.1".to_string());

        Ok(StatbookConfig {
            api_key,
            password,
            base_url,
        })
    }
}
