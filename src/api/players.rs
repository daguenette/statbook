use crate::{
    client::StatbookClient,
    error::Result,
    models::{Article, FetchStrategy, NewsQuery, PlayerStats, PlayerSummaryResult},
    utils::to_dash_case,
};

pub async fn get_player_stats(client: &StatbookClient, name: &str) -> Result<PlayerStats> {
    let dash_name = to_dash_case(name);
    client.stats_provider().fetch_player_stats(&dash_name).await
}

pub async fn get_player_news(client: &StatbookClient, query: &NewsQuery) -> Result<Vec<Article>> {
    client.news_provider().fetch_player_news(query).await
}

/// Fetch player summary with configurable strategy
pub async fn get_player_summary(
    client: &StatbookClient,
    name: &str,
    strategy: FetchStrategy,
) -> Result<PlayerSummaryResult> {
    let dash_name = to_dash_case(name);

    match strategy {
        FetchStrategy::StatsOnly => {
            let stats = client
                .stats_provider()
                .fetch_player_stats(&dash_name)
                .await?;
            Ok(PlayerSummaryResult {
                player_stats: stats,
                news_result: Ok(vec![]),
            })
        }
        FetchStrategy::NewsOnly => {
            let query = NewsQuery::for_player(&dash_name);
            let news = client.news_provider().fetch_player_news(&query).await?;
            // Return empty stats for news-only requests
            Ok(PlayerSummaryResult {
                player_stats: PlayerStats {
                    first_name: String::new(),
                    last_name: String::new(),
                    primary_position: String::new(),
                    jersey_number: 0,
                    current_team: String::new(),
                    injury: String::new(),
                    rookie: false,
                    games_played: 0,
                },
                news_result: Ok(news),
            })
        }
        FetchStrategy::Both { fail_on_news_error } => {
            get_player_summary_concurrent(client, &dash_name, fail_on_news_error).await
        }
    }
}

/// Fetch player summary with concurrent API calls
pub async fn get_player_summary_concurrent(
    client: &StatbookClient,
    name: &str,
    fail_on_news_error: bool,
) -> Result<PlayerSummaryResult> {
    let dash_name = to_dash_case(name);
    let query = NewsQuery::for_player(&dash_name);

    let (stats_result, news_result) = tokio::join!(
        client.stats_provider().fetch_player_stats(&dash_name),
        client.news_provider().fetch_player_news(&query)
    );

    let stats = stats_result?;

    if fail_on_news_error {
        let news = news_result?;
        Ok(PlayerSummaryResult {
            player_stats: stats,
            news_result: Ok(news),
        })
    } else {
        Ok(PlayerSummaryResult {
            player_stats: stats,
            news_result,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_mock_client;

    #[tokio::test]
    async fn test_get_player_stats_mock() {
        let client = create_mock_client();
        let stats = get_player_stats(&client, "josh-allen").await.unwrap();

        assert_eq!(stats.first_name, "Josh");
        assert_eq!(stats.last_name, "Allen");
        assert_eq!(stats.primary_position, "QB");
        assert_eq!(stats.jersey_number, 17);
        assert_eq!(stats.current_team, "BUF");
    }

    #[tokio::test]
    async fn test_get_player_news_mock() {
        let client = create_mock_client();
        let query = NewsQuery::for_player("josh-allen");
        let news = get_player_news(&client, &query).await.unwrap();

        assert!(!news.is_empty());
        assert!(news[0].title.contains("Josh Allen"));
    }

    #[tokio::test]
    async fn test_get_player_summary_concurrent_mock() {
        let client = create_mock_client();
        let result = get_player_summary_concurrent(&client, "josh-allen", false)
            .await
            .unwrap();

        assert_eq!(result.player_stats.first_name, "Josh");
        assert_eq!(result.player_stats.last_name, "Allen");
        assert!(result.news_result.is_ok());

        let news = result.news_result.unwrap();
        assert!(!news.is_empty());
    }

    #[tokio::test]
    async fn test_get_player_summary_strategies_mock() {
        let client = create_mock_client();

        // Test stats only
        let stats_only = get_player_summary(&client, "josh-allen", FetchStrategy::StatsOnly)
            .await
            .unwrap();
        assert_eq!(stats_only.player_stats.first_name, "Josh");
        assert!(stats_only.news_result.unwrap().is_empty());

        // Test both with no failure on news error
        let both = get_player_summary(
            &client,
            "josh-allen",
            FetchStrategy::Both {
                fail_on_news_error: false,
            },
        )
        .await
        .unwrap();
        assert_eq!(both.player_stats.first_name, "Josh");
        assert!(both.news_result.is_ok());
    }
}
