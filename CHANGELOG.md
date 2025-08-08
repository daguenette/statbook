# Changelog

## [0.0.3] - 2025-08-08

### Breaking Changes

- **Season Parameter Support**: Player statistics functions now require season parameters
  - `get_player_stats()` signature: `get_player_stats(&client, name, year_range, season)`
  - `get_player_summary()` signature: `get_player_summary(&client, name, year_range, season)`
  - All `StatsProvider` implementations must support season parameter

- **API Return Types**: Improved domain modeling with better type consistency
  - `get_player_news()` now returns `Result<PlayerNews>` instead of `Result<Vec<Article>>`
  - `get_player_summary()` now returns `Result<PlayerSummary>` instead of `Result<PlayerSummaryResult>`
  - `PlayerSummaryResult` renamed to `PlayerSummary` for better naming consistency

- **PlayerSummary Structure**: Simplified to essential fields only
  - Now contains direct fields: `first_name`, `last_name`,
  `primary_position`, `current_team`, `jersey_number`, `games_played`, `news`
  - Removed nested `player_stats`
  field - access fields directly on `PlayerSummary`
  - Removed `news_result: Result<PlayerNews, Error>` -
  now `news: Vec<Article>` with graceful failure (empty vec)
  - Detailed fields like `injury` and `rookie`
  moved to `get_player_stats()` for full information

- **get_player_summary() Function**: Removed FetchStrategy parameter for simplicity
  - OLD: `get_player_summary(&client, "name", FetchStrategy::Both { fail_on_news_error: false })`
  - NEW: `get_player_summary(&client, "name", year_range, season)` - always fetches both stats and news concurrently
  - Use `get_player_stats()` for stats-only, `get_player_news()` for news-only
  - FetchStrategy enum preserved for future use in `get_player_report()` functionality

### Added

- **Season Parameter Support**: Query player statistics for specific seasons
  - New `Season` enum with variants: `Regular`, `Playoffs`, `Current`, `Latest`, `Upcoming`
  - Year range support for custom date ranges (e.g., "2023-2024-regular")
  - `PlayerStats` now includes `season: String` field with season information
  - Enhanced API flexibility for historical and current season data

- **PlayerNews struct**: Wraps news articles with metadata
  - Contains `articles: Vec<Article>`, `query: NewsQuery`, and optional `total_count`
  - Provides `len()`, `is_empty()`, and `with_total_count()` methods
  - Better extensibility for future enhancements

### Improved

- **PlayerSummary**: Now a true "summary" with essential player info only
  - Flattened structure for easier access
  - Graceful news failure handling (empty vec instead of Result)
  - Clear separation: use `get_player_summary()` for
  overview, `get_player_stats()` for detailed info

- **Simplified API**: Removed FetchStrategy complexity from get_player_summary()
  - Each function now has one clear purpose
  - Better separation of concerns
  - Reduced cognitive load for developers

### Migration Guide

```rust
// OLD (v0.0.2):
let news: Vec<Article> = get_player_news(&client, &query).await?;
let summary: PlayerSummaryResult = get_player_summary(&client, "player", strategy).await?;
let stats = get_player_stats(&client, "player").await?;
println!("{}", summary.player_stats.first_name);
match summary.news_result { ... }

// NEW (v0.0.3):
use statbook::Season;

let news: PlayerNews = get_player_news(&client, &query).await?;
let articles: &Vec<Article> = &news.articles; // Access articles

// Season parameter now required:
let stats = get_player_stats(&client, "player", None, &Season::Regular).await?;
let summary = get_player_summary(&client, "player", None, &Season::Regular).await?;

// Season examples:
let regular_stats = get_player_stats(&client, "josh-allen", None, &Season::Regular).await?;
let playoff_stats = get_player_stats(&client, "josh-allen", Some((2023, 2024)), &Season::Playoffs).await?;
println!("Season: {}", stats.season); // New season field

println!("{}", summary.first_name); // Direct field access
println!("News: {}", summary.news.len()); // Direct news access, no Result handling
```

### Internal Changes

- Updated `NewsProvider` trait to return `PlayerNews`
- Updated all provider implementations
- Updated mock providers for testing
- Improved models organization

## [0.0.2] - 2025-08-06

### Added

- Complete sports statistics and news data library
- NFL player statistics via MySportsFeeds API
- News articles via NewsAPI integration
- Concurrent API calls with `tokio::join!`
- Comprehensive error handling with `thiserror`
- Provider pattern for extensible data sources
- Mock providers for testing
- Builder pattern configuration
- Environment variable configuration
- Multiple fetch strategies (StatsOnly, NewsOnly, Both)
- Graceful partial failure handling
- NewsAPI free tier compatibility

### Features

- `StatbookClient` - Main client interface
- `get_player_stats()` - Fetch player statistics
- `get_player_news()` - Fetch news articles
- `get_player_summary()` - Fetch both with strategies
- Custom provider support via traits
- Comprehensive documentation and examples

### Documentation

- Complete API documentation
- Usage examples in `examples/`
- Integration and unit tests
- README with detailed setup instructions

## [0.0.1] - Initial Development

- Basic project structure
