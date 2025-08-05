use statbook::{api::players::get_player_by_name, StatbookClient, StatbookConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = StatbookConfig::builder()
        .api_key("your-api-key") // Replace with your actual API key
        .password("your-password") // Replace with your actual password
        .build()?;

    let client = StatbookClient::new(config);

    // Try to get multiple players
    let players = vec!["tom-brady", "aaron-rodgers", "patrick-mahomes"];

    for player_name in players {
        println!("\nSearching for: {player_name}");

        match get_player_by_name(&client, player_name).await {
            Ok(Some(player)) => {
                println!(
                    "{} {} - {} ({})",
                    player.first_name,
                    player.last_name,
                    player.primary_position,
                    player.current_team
                );
            }
            Ok(None) => {
                println!("No player found");
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }

    Ok(())
}

