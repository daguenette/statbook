/// Basic usage example demonstrating different ways to fetch player data.
///
/// This example shows:
/// - Creating a client from environment variables
/// - Fetching complete player summaries
/// - Getting stats-only data (faster)
/// - Getting news-only data
/// - Using concurrent fetching with graceful error handling
///
/// To run this example:
/// ```bash
/// export STATS_API_KEY="your-mysportsfeeds-api-key"
/// export NEWS_API_KEY="your-newsapi-key"
/// cargo run --example basic_usage
/// ```
use statbook::{
    api::players::{get_player_news, get_player_stats, get_player_summary},
    NewsQuery, Season, StatbookClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variables
    // Requires STATS_API_KEY and NEWS_API_KEY to be set
    dotenv::dotenv().ok(); // Load .env file if present
    let client = StatbookClient::from_env()?;

    // Example 1: Stats-only fetch
    // Use this when you only need player statistics
    println!("\n=== Example 1: Stats Only (Fastest) ===");
    let stats = get_player_stats(&client, "josh-allen", None, &Season::Regular).await?;
    println!(
        "⚡ {} {} - {} #{} | {} games (Season: {})",
        stats.first_name,
        stats.last_name,
        stats.primary_position,
        stats.jersey_number,
        stats.games_played,
        stats.season
    );

    // Example 2: News-only fetch
    // Use this for news aggregation or when stats aren't needed
    println!("\n=== Example 2: News Only ===");
    let query = NewsQuery::for_player("Josh Allen").with_page_size(3);
    let news = get_player_news(&client, &query).await?;
    println!("Found {} news articles:", news.len());

    for (i, article) in news.articles.iter().take(2).enumerate() {
        println!("  {}. {}", i + 1, article.title);
        println!("     Published: {}", article.published_at);
        if !article.description.is_empty() {
            println!("     {}", article.description);
        }
        println!();
    }

    // Example 3: Player summary with essential info and news
    // This is the recommended approach for getting a quick overview
    println!("\n=== Example 3: Player Summary (Recommended) ===");
    let result = get_player_summary(&client, "josh-allen", None, &Season::Regular).await?;

    // Stats are always available if we get here
    println!(
        "Player: {} {} - {} ({})",
        result.first_name, result.last_name, result.primary_position, result.current_team
    );

    // News is gracefully handled - empty vec if failed
    if result.news.is_empty() {
        println!("No news articles available (may have failed gracefully)");
    } else {
        println!("Successfully fetched {} news articles", result.news.len());
        if let Some(latest) = result.news.first() {
            println!("   Latest: {}", latest.title);
        }
    }

    // Different function comparison
    println!("\n=== Function Comparison ===");

    // Time the different functions (rough timing)
    use std::time::Instant;

    let start = Instant::now();
    let _stats_only = get_player_stats(&client, "josh-allen", None, &Season::Regular).await?;
    println!("get_player_stats took: {:?}", start.elapsed());

    let start = Instant::now();
    let _summary = get_player_summary(&client, "josh-allen", None, &Season::Regular).await?;
    println!(
        "get_player_summary (concurrent) took: {:?}",
        start.elapsed()
    );

    // Example 4: Different seasons
    println!("\n=== Example 4: Different Seasons ===");
    let regular_stats =
        get_player_stats(&client, "josh-allen", Some((2023, 2024)), &Season::Regular).await?;
    println!(
        "Regular season 2023-2024: {} games",
        regular_stats.games_played
    );

    let playoff_stats =
        get_player_stats(&client, "josh-allen", Some((2023, 2024)), &Season::Playoffs).await?;
    println!(
        "Playoff season 2023-2024: {} games",
        playoff_stats.games_played
    );

    println!("\nAll examples completed successfully!");
    println!("Tip: Use get_player_stats() for detailed info, get_player_summary() for overview");

    Ok(())
}
