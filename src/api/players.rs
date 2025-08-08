use crate::{
    client::StatbookClient,
    error::Result,
    models::{NewsQuery, PlayerNews, PlayerStats, PlayerSummary, Season},
    utils::to_dash_case,
};

pub async fn get_player_stats(
    client: &StatbookClient,
    name: &str,
    year_range: Option<(i64, i64)>,
    season: &Season,
) -> Result<PlayerStats> {
    let dash_name = to_dash_case(name);

    let season_param: String = match year_range {
        Some((start, end)) => season.format_with_years(start, end),
        None => season.as_str().to_string(),
    };

    client
        .stats_provider()
        .fetch_player_stats(&dash_name, &season_param)
        .await
}

pub async fn get_player_news(client: &StatbookClient, query: &NewsQuery) -> Result<PlayerNews> {
    client.news_provider().fetch_player_news(query).await
}

pub async fn get_player_summary(
    client: &StatbookClient,
    name: &str,
    year_range: Option<(i64, i64)>,
    season: &Season,
) -> Result<PlayerSummary> {
    let dash_name = to_dash_case(name);
    let query = NewsQuery::for_player(&dash_name);
    let season_param: String = match year_range {
        Some((start, end)) => season.format_with_years(start, end),
        None => season.as_str().to_string(),
    };

    let (stats_result, news_result) = tokio::join!(
        client
            .stats_provider()
            .fetch_player_stats(&dash_name, &season_param),
        client.news_provider().fetch_player_news(&query)
    );

    let stats = stats_result?;
    let news = news_result.map(|n| n.articles).unwrap_or_default();

    Ok(PlayerSummary {
        first_name: stats.first_name,
        last_name: stats.last_name,
        primary_position: stats.primary_position,
        current_team: stats.current_team,
        jersey_number: stats.jersey_number,
        games_played: stats.games_played,
        news,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_mock_client;

    #[tokio::test]
    async fn test_get_player_stats_mock() {
        let client = create_mock_client();
        let stats = get_player_stats(&client, "josh-allen", None, &Season::Regular)
            .await
            .unwrap();

        assert_eq!(stats.first_name, "Josh");
        assert_eq!(stats.last_name, "Allen");
        assert_eq!(stats.primary_position, "QB");
        assert_eq!(stats.jersey_number, 17);
        assert_eq!(stats.current_team, "BUF");
        assert_eq!(stats.season, "regular");
    }

    #[tokio::test]
    async fn test_get_player_news_mock() {
        let client = create_mock_client();
        let query = NewsQuery::for_player("josh-allen");
        let news = get_player_news(&client, &query).await.unwrap();

        assert!(!news.is_empty());
        assert!(news.articles[0].title.contains("Josh Allen"));
    }

    #[tokio::test]
    async fn test_get_player_summary_mock() {
        let client = create_mock_client();
        let result = get_player_summary(&client, "josh-allen", None, &Season::Regular)
            .await
            .unwrap();

        assert_eq!(result.first_name, "Josh");
        assert_eq!(result.last_name, "Allen");
        assert!(!result.news.is_empty());
    }

    #[tokio::test]
    async fn test_get_player_summary_with_news_mock() {
        let client = create_mock_client();

        // Test that summary always fetches both stats and news
        let summary = get_player_summary(&client, "josh-allen", None, &Season::Regular)
            .await
            .unwrap();

        // Should have player info
        assert_eq!(summary.first_name, "Josh");
        assert_eq!(summary.last_name, "Allen");
        assert_eq!(summary.primary_position, "QB");
        assert_eq!(summary.current_team, "BUF");

        // Should have news articles
        assert!(!summary.news.is_empty());
    }

    #[tokio::test]
    async fn test_season_parameter_integration() {
        let client = create_mock_client();

        // Test different seasons
        let regular_stats = get_player_stats(&client, "josh-allen", None, &Season::Regular)
            .await
            .unwrap();
        assert_eq!(regular_stats.season, "regular");

        let playoff_stats = get_player_stats(&client, "josh-allen", None, &Season::Playoffs)
            .await
            .unwrap();
        assert_eq!(playoff_stats.season, "playoff");

        // Test with year range
        let stats_with_years =
            get_player_stats(&client, "josh-allen", Some((2023, 2024)), &Season::Regular)
                .await
                .unwrap();
        assert_eq!(stats_with_years.season, "2023-2024-regular");

        // Test summary with season
        let summary =
            get_player_summary(&client, "josh-allen", Some((2023, 2024)), &Season::Playoffs)
                .await
                .unwrap();
        assert_eq!(summary.first_name, "Josh");
        assert_eq!(summary.last_name, "Allen");
    }
}
