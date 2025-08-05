use statbook::{api::players::get_player_by_name, StatbookClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variables
    // Make sure to set STATBOOK_API_KEY and STATBOOK_PASSWORD
    let client = StatbookClient::from_env()?;

    // Get player data
    match get_player_by_name(&client, "josh-allen").await? {
        Some(player) => {
            println!("Name: {} {}", player.first_name, player.last_name);
            println!("Position: {}", player.primary_position);
            println!("Jersey: #{}", player.jersey_number);
            println!("Team: {}", player.current_team);
            println!("Rookie: {}", player.rookie);
            println!("Games Played: {}", player.games_played);

            if !player.injury.is_empty() {
                println!("Injury Status: {}", player.injury);
            }
        }
        None => {
            println!("No player found with that name");
        }
    }

    Ok(())
}

