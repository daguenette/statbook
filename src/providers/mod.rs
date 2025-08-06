pub mod mock_providers;
pub(crate) mod news_provider;
pub(crate) mod stats_provider;

// Re-export only the traits and mock providers, not concrete implementations
pub use mock_providers::{MockNewsProvider, MockStatsProvider};
pub use news_provider::NewsProvider;
pub use stats_provider::StatsProvider;

// Keep concrete implementations internal but accessible within crate
pub(crate) use news_provider::NewsApiProvider;
pub(crate) use stats_provider::MySportsStatsProvider;
