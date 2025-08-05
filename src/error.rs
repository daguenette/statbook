use std::fmt;

#[derive(Debug)]
pub enum StatbookError {
    MissingApiKey,
    MissingPassword,
    HttpError(reqwest::Error),
    ParseError(serde_json::Error),
    PlayerNotFound,
}

impl fmt::Display for StatbookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatbookError::MissingApiKey => {
                write!(f, "Missing API key. Set STATBOOK_API_KEY environment variable or provide via client configuration")
            }
            StatbookError::MissingPassword => {
                write!(f, "Missing password. Set STATBOOK_PASSWORD environment variable or provide via client configuration")
            }
            StatbookError::HttpError(e) => write!(f, "HTTP request failed: {e}"),
            StatbookError::ParseError(e) => write!(f, "Failed to parse response: {e}"),
            StatbookError::PlayerNotFound => write!(f, "No player data found"),
        }
    }
}

impl std::error::Error for StatbookError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StatbookError::HttpError(e) => Some(e),
            StatbookError::ParseError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for StatbookError {
    fn from(error: reqwest::Error) -> Self {
        StatbookError::HttpError(error)
    }
}

impl From<serde_json::Error> for StatbookError {
    fn from(error: serde_json::Error) -> Self {
        StatbookError::ParseError(error)
    }
}

pub type Result<T> = std::result::Result<T, StatbookError>;

