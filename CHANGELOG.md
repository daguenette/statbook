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
