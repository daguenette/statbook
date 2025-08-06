pub(crate) mod news_parser;
mod player;
pub(crate) mod player_parser;
mod result_types;

// Re-export public types
pub use player::{Article, PlayerSummary};
pub use result_types::{FetchStrategy, NewsQuery, PlayerStats, PlayerSummaryResult};
