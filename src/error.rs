/// Errors that can occur when using the Statbook library.
///
/// This enum represents all possible error conditions that can arise
/// when fetching sports statistics and news data. Each variant provides
/// specific context about what went wrong.
///
/// # Examples
///
/// ```rust
/// use statbook::{StatbookClient, StatbookError};
///
/// # async fn example() {
/// match StatbookClient::from_env() {
///     Ok(client) => {
///         // Use the client
///     }
///     Err(StatbookError::MissingApiKey { key }) => {
///         eprintln!("Please set the {} environment variable", key);
///     }
///     Err(StatbookError::Config(msg)) => {
///         eprintln!("Configuration error: {}", msg);
///     }
///     Err(e) => {
///         eprintln!("Other error: {}", e);
///     }
/// }
/// # }
/// ```
#[derive(Debug, thiserror::Error)]
pub enum StatbookError {
    /// An API key is missing or empty.
    ///
    /// This error occurs when a required API key is not provided
    /// in the configuration or environment variables.
    #[error("Missing API key: {key}")]
    MissingApiKey {
        /// The name of the missing API key (e.g., "STATS_API_KEY")
        key: String,
    },

    /// The requested player was not found.
    ///
    /// This error occurs when searching for a player that doesn't exist
    /// in the statistics provider's database.
    #[error("Player '{name}' not found")]
    PlayerNotFound {
        /// The name of the player that was not found
        name: String,
    },

    /// A network-related error occurred.
    ///
    /// This error wraps HTTP client errors, including connection timeouts,
    /// DNS resolution failures, and other network issues.
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// Failed to parse JSON response.
    ///
    /// This error occurs when the API returns malformed JSON or when
    /// the response structure doesn't match expected format.
    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),

    /// The statistics API returned an error.
    ///
    /// This error occurs when the stats provider API returns a non-success
    /// HTTP status code, such as 401 (unauthorized) or 429 (rate limited).
    #[error("Stats API error: {status} - {message}")]
    StatsApi {
        /// HTTP status code returned by the API
        status: u16,
        /// Error message from the API response
        message: String,
    },

    /// The news API returned an error.
    ///
    /// This error occurs when the news provider API returns a non-success
    /// HTTP status code, such as 401 (unauthorized) or 429 (rate limited).
    #[error("News API error: {status} - {message}")]
    NewsApi {
        /// HTTP status code returned by the API
        status: u16,
        /// Error message from the API response
        message: String,
    },

    /// A configuration error occurred.
    ///
    /// This error occurs when there are issues with the client configuration,
    /// such as invalid URLs or malformed settings.
    #[error("Configuration error: {0}")]
    Config(String),

    /// A validation error occurred.
    ///
    /// This error occurs when input validation fails, such as when
    /// API keys don't meet format requirements.
    #[error("Validation error: {0}")]
    Validation(String),
}

/// A specialized `Result` type for Statbook operations.
///
/// This is a type alias for `std::result::Result<T, StatbookError>` that
/// simplifies function signatures throughout the library.
///
/// # Examples
///
/// ```rust
/// use statbook::{Result, StatbookClient};
///
/// fn create_client() -> Result<StatbookClient> {
///     StatbookClient::from_env()
/// }
/// ```
pub type Result<T> = std::result::Result<T, StatbookError>;
