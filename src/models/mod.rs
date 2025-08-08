mod fetch;
mod news;
mod player;
pub(crate) mod parsers;

// Re-export public types
pub use fetch::FetchStrategy;
pub use news::{Article, NewsQuery};
pub use player::{PlayerStats, PlayerSummaryResult};
