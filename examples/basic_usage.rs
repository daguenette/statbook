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
    FetchStrategy, NewsQuery, StatbookClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variables
    // Requires STATS_API_KEY and NEWS_API_KEY to be set
    dotenv::dotenv().ok(); // Load .env file if present
    let client = StatbookClient::from_env()?;

    // Example 1: Stats-only fetch (fastest option)
    // Use this when you only need player statistics
    println!("\n=== Example 1: Stats Only (Fastest) ===");
    let stats = get_player_stats(&client, "josh-allen").await?;
    println!(
        "⚡ {} {} - {} #{} | {} games",
        stats.first_name,
        stats.last_name,
        stats.primary_position,
        stats.jersey_number,
        stats.games_played
    );

    // Example 2: News-only fetch
    // Use this for news aggregation or when stats aren't needed
    println!("\n=== Example 2: News Only ===");
    let query = NewsQuery::for_player("Josh Allen").with_page_size(3);
    let news = get_player_news(&client, &query).await?;
    println!("Found {} news articles:", news.len());

    for (i, article) in news.iter().take(2).enumerate() {
        println!("  {}. {}", i + 1, article.title);
        println!("     Published: {}", article.published_at);
        if !article.description.is_empty() {
            println!("     {}", article.description);
        }
        println!();
    }

    // Example 3: Concurrent fetch with graceful error handling
    // This is the recommended approach for most applications
    println!("\n=== Example 3: Concurrent Fetch (Recommended) ===");
    let result = get_player_summary(
        &client,
        "josh-allen",
        FetchStrategy::Both {
            fail_on_news_error: false, // Continue even if news fails
        },
    )
    .await?;

    // Stats are always available if we get here
    let stats = &result.player_stats;
    println!(
        "Player: {} {} - {} ({})",
        stats.first_name, stats.last_name, stats.primary_position, stats.current_team
    );

    // News might have failed - handle gracefully
    match result.news_result {
        Ok(articles) => {
            println!("Successfully fetched {} news articles", articles.len());
            if let Some(latest) = articles.first() {
                println!("   Latest: {}", latest.title);
            }
        }
        Err(e) => {
            println!("News fetch failed (stats still available): {}", e);
            println!("This is normal - news API might be rate limited or down");
        }
    }

    // Different fetch strategies comparison
    println!("\n=== Strategy Comparison ===");

    // Time the different strategies (rough timing)
    use std::time::Instant;

    let start = Instant::now();
    let _stats_only = get_player_summary(&client, "josh-allen", FetchStrategy::StatsOnly).await?;
    println!("StatsOnly took: {:?}", start.elapsed());

    let start = Instant::now();
    let _both = get_player_summary(
        &client,
        "josh-allen",
        FetchStrategy::Both {
            fail_on_news_error: false,
        },
    )
    .await?;
    println!("Both (concurrent) took: {:?}", start.elapsed());

    println!("\nAll examples completed successfully!");
    println!("Tip: Use StatsOnly for fastest response, Both for complete data");

    Ok(())
}
