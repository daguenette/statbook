use crate::{
    client::StatbookClient,
    providers::{MockNewsProvider, MockStatsProvider},
};
use std::sync::Arc;

/// Initialize a test client with real API credentials if available
pub fn init_integration_test_client() -> Option<StatbookClient> {
    if std::env::var("INTEGRATION_TESTS").is_ok() {
        StatbookClient::from_env().ok()
    } else {
        None
    }
}

/// Skip test if no credentials are available
pub fn skip_if_no_credentials() -> Option<StatbookClient> {
    match init_integration_test_client() {
        Some(client) => Some(client),
        None => {
            println!("Skipping integration test - no credentials provided");
            println!("Set INTEGRATION_TESTS=1 and provide STATS_API_KEY and NEWS_API_KEY to run integration tests");
            None
        }
    }
}

/// Create a test client with mock providers
pub fn create_mock_client() -> StatbookClient {
    let stats_provider = Arc::new(MockStatsProvider::with_defaults());
    let news_provider = Arc::new(MockNewsProvider::with_defaults());

    StatbookClient::with_providers(stats_provider, news_provider)
}

/// Create a test client with custom mock providers
pub fn create_custom_mock_client(
    stats_provider: MockStatsProvider,
    news_provider: MockNewsProvider,
) -> StatbookClient {
    let stats_provider = Arc::new(stats_provider);
    let news_provider = Arc::new(news_provider);

    StatbookClient::with_providers(stats_provider, news_provider)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::players::{get_player_news, get_player_stats};
    use crate::models::NewsQuery;

    #[tokio::test]
    async fn test_mock_client_stats() {
        let client = create_mock_client();
        let stats = get_player_stats(&client, "josh-allen").await.unwrap();

        assert_eq!(stats.first_name, "Josh");
        assert_eq!(stats.last_name, "Allen");
        assert_eq!(stats.primary_position, "QB");
    }

    #[tokio::test]
    async fn test_mock_client_news() {
        let client = create_mock_client();
        let query = NewsQuery::for_player("josh-allen");
        let news = get_player_news(&client, &query).await.unwrap();

        assert!(!news.is_empty());
        assert!(news[0].title.contains("Josh Allen"));
    }

    #[tokio::test]
    async fn test_integration_client_creation() {
        // This test just verifies the integration test helper works
        match skip_if_no_credentials() {
            Some(client) => {
                println!("Integration test running with real API credentials");
                // Actually test the APIs
                match get_player_stats(&client, "Tom Brady").await {
                    Ok(stats) => {
                        println!(
                            "Stats API working: {} {}",
                            stats.first_name, stats.last_name
                        );
                        assert!(!stats.first_name.is_empty());
                    }
                    Err(e) => {
                        println!("Stats API failed: {}", e);
                        panic!("Stats API integration test failed: {}", e);
                    }
                }

                let query = NewsQuery::for_player("josh-allen");
                match get_player_news(&client, &query).await {
                    Ok(articles) => {
                        println!("News API working: {} articles found", articles.len());
                        // News might be empty, that's ok
                    }
                    Err(e) => {
                        println!("News API failed: {}", e);
                        panic!("News API integration test failed: {}", e);
                    }
                }
            }
            None => {
                println!("⏭️  Skipping integration test - no credentials provided");
            }
        }
    }
}
